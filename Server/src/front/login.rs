//! # Login path `/api/login`
//!
//! This handles user logins and locally exposes [`db_new_user_key`] to allow a new user to return an user key on
//! success


use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};
use actix_files::NamedFile;

use actix_web::{get, http::header::ContentType, post, web, Error, HttpMessage, HttpRequest, HttpResponse, Either};
use actix_web::http::StatusCode;
use chrono::Local;
use deadpool_postgres::{Client, Manager, Pool};
use json::JsonValue;
use rand::SeedableRng;
use rand_chacha::rand_core::block::BlockRngCore;
use tokio_postgres::Row;

use crate::{db::ApiError, front::prelude::Admin, logger::Logger, logger_wrap};
use crate::front::prelude::AdminSubmission;

/// Get a new user key from a given UUID and uname
pub(in crate::front) async fn db_new_admin_key(
	client: &Client,
	email: &str,
	password: &str,
	uuid: i32,
	logger: &web::Data<Logger<'_>>,
	req: &HttpRequest,
) -> Either<JsonValue, StatusCode> {
	let mut hasher = DefaultHasher::new();
	email.hash(&mut hasher);
	uuid.hash(&mut hasher);
	password.hash(&mut hasher);
	Local::now().hash(&mut hasher);
	let mut rng = rand_chacha::ChaCha20Core::seed_from_u64(hasher.finish());
	let mut r = Default::default();
	rng.generate(&mut r);
	let user_key = r
		.as_ref()
		.iter()
		.fold(String::new(), |acc, t| format!("{}{:08x}", acc, t));
	let end = Local::now() + chrono::Days::new(7);
	
	let stmt = client.prepare(include_str!("../../sql/admin/new_key.sql")).await.unwrap();

	if let Err(e) = client.query(&stmt, &[&user_key, &uuid, &end]).await {
		logger_wrap!(logger.error, req, format!("{}:{} Admin key insertion error {:?}", file!(), line!(), e));
		return Either::Right(StatusCode::INTERNAL_SERVER_ERROR)
	};
	
	Either::Left(json::object! {
		cookie: json::object! {
			value: user_key,
			time: end.to_rfc3339()
		},
		redirect: "dashboard"
	})
}

/// User login path
#[get("/login")]
pub async fn front_login(req: HttpRequest, db_pool: web::Data<Pool>, logger: web::Data<Logger<'_>>) -> HttpResponse {
	if let Some(cookie) = req.cookie("login") {
		match db_pool.get().await {
			Ok(client) => {
				let stmt = client.prepare(include_str!("../../sql/admin/auth_key.sql")).await.unwrap();
				match client.query(&stmt, &[&cookie.value()]).await {
					Ok(rows) => {
						if !rows.is_empty() {
							return HttpResponse::Found().insert_header(("Location", "/dashboard")).finish()
						}
					}
					Err(e) => {
						logger_wrap!(logger.error, req, format!("Cookie check query error - {:?}", e))
					}
				}
			},
			Err(e) => {
				logger_wrap!(logger.error, req, format!("Error connecting to db on login cookie check - {:?}", e))
			}
		}
	}
	NamedFile::open("front/login.html").expect("Could not find login file (login.html)").into_response(&req)
}

/// User login path
#[post("/login")]
pub async fn front_login_post(
	req: HttpRequest,
	user: web::Json<AdminSubmission>,
	db_pool: web::Data<Pool>,
	logger: web::Data<Logger<'_>>,
) -> HttpResponse {
	let client = match db_pool.get().await {
		Ok(client) => client,
		Err(e) => {
			logger_wrap!(logger.info, req, format!("Client connection error {:?}", e));
			return HttpResponse::InternalServerError().finish()
		}
	};
	let stmt = client.prepare(include_str!("../../sql/admin/login.sql")).await.unwrap();
	let uuid = match client.query(&stmt, &[&user.email, &user.password]).await {
		Ok(mut rows) => {
			if let Some(row) = rows.pop() {
				match row.try_get::<_, i32>(0) {
					Ok(uuid) => uuid,
					Err(e) => {
						logger_wrap!(logger.info, req, format!("Query result get error {:?}", e));
						return HttpResponse::InternalServerError().finish()
					}
				}
			} else {
				return HttpResponse::Forbidden().finish()
			}
		}
		Err(err) => {
			logger_wrap!(logger.error, req, format!("Login check query error {:?}", err));
			return HttpResponse::InternalServerError().finish()
		}
	};
	match db_new_admin_key(&client, &user.email, &user.password, uuid, &logger, &req).await {
		Either::Left(cookie) => {
			HttpResponse::Ok().insert_header(("content-type", "application/json")).body(cookie.to_string())
		}
		Either::Right(err) => {
			HttpResponse::new(err)
		}
	}
}
