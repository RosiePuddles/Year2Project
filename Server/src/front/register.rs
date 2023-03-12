//! # Frontend register and register verification
//!
//! This handles frontend user registration and registration credential verification

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

use actix_files::NamedFile;
use actix_web::{get, http::header::ContentType, post, web, web::Json, Error, HttpMessage, HttpRequest, HttpResponse, Responder, Either};
use actix_web::http::{header, StatusCode};
use chrono::Local;
use deadpool_postgres::{Client, Manager, Pool};
use json::JsonValue;
use rand::SeedableRng;
use rand_chacha::rand_core::block::BlockRngCore;
use regex::Regex;
use tokio_postgres::Row;

use crate::{
	db::ApiError,
	front::prelude::{Admin, AdminSubmission},
	logger::Logger,
	logger_wrap,
};
use crate::front::login::db_new_admin_key;

// async fn db_login(client: &Client, uname: String, logger: &web::Data<Logger<'_>>) -> Result<JsonValue, ApiError> {
// 	macro_rules! get_wrapper {
// 		($t: expr) => {
// 			match $t {
// 				Ok(t) => t,
// 				Err(e) => {
// 					logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
// 					return Err(ApiError::ServerError)
// 				}
// 			}
// 		};
// 	}
// 	// check for existing uname and get uuid
// 	let stmt = client.prepare(&include_str!("../../sql/api/user_check.sql")).await.unwrap();
//
// 	let uuid = match client.query(&stmt, &[&uname]).await {
// 		Ok(mut rows) => {
// 			if let Some(row) = rows.pop() {
// 				// don't need to check for multiple rows because uname is unique
// 				get_wrapper!(row.try_get::<_, i32>(0))
// 			} else {
// 				logger_wrap!(logger.info(format!(
// 					"{}:{} Login with unknown uname requested {:?}",
// 					file!(),
// 					line!(),
// 					uname
// 				)));
// 				return Err(ApiError::Gone)
// 			}
// 		}
// 		Err(e) => {
// 			logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
// 			return Err(ApiError::ServerError)
// 		}
// 	};
//
// 	db_new_user_key(client, uuid, uname, logger).await
// }
//
/// Performs checks to ensure no conflicts and
async fn db_new_admin(
	client: &Client, email: &String, password: &String, logger: &web::Data<Logger<'_>>, req: &HttpRequest
) -> Either<JsonValue, StatusCode> {
	macro_rules! get_wrapper {
		($t: expr) => {
			match $t {
				Ok(t) => t,
				Err(e) => {
					logger_wrap!(logger.error, req, format!("{}:{} Admin creation {:?}", file!(), line!(), e.to_string()));
					return Either::Right(StatusCode::INTERNAL_SERVER_ERROR)
				}
			}
		};
	}
	// check no-conflict for email
	let stmt = client.prepare(include_str!("../../sql/admin/no_conflict.sql")).await.unwrap();
	match client.query(&stmt, &[email]).await {
		Ok(rows) => {
			if !rows.is_empty() {
				return Either::Right(StatusCode::CONFLICT)
			}
		}
		Err(err) => {
			logger_wrap!(logger.error, req, format!("New admin conflict check error {:?}", err));
			return Either::Right(StatusCode::INTERNAL_SERVER_ERROR)
		}
	}
	// insert new user
	let stmt = client.prepare(include_str!("../../sql/admin/new_admin.sql")).await.unwrap();
	let uuid = match client.query(&stmt, &[email, password]).await {
		Ok(mut rows) => {
			if let Some(row) = rows.pop() {
				match row.try_get::<_, i32>(0) {
					Ok(uuid) => uuid,
					Err(e) => {
						logger_wrap!(logger.error, req, format!("{}:{} New admin creation error {:?}", file!(), line!(), e));
						return Either::Right(StatusCode::INTERNAL_SERVER_ERROR)
					}
				}
			} else {
				logger_wrap!(logger.error, req, format!("{}:{} New admin creation error. No uuid returned", file!(), line!()));
				return Either::Right(StatusCode::INTERNAL_SERVER_ERROR)
			}
		}
		Err(err) => {
			logger_wrap!(logger.error, req, format!("{}:{} New admin creation error {:?}", file!(), line!(), err));
			return Either::Right(StatusCode::INTERNAL_SERVER_ERROR)
		}
	};
	
	db_new_admin_key(client, email, password, uuid, logger, req).await
}

#[get("/register")]
pub async fn front_register() -> impl Responder { NamedFile::open("front/register.html") }

#[post("/register")]
pub async fn front_register_post(
	req: HttpRequest,
	user: Json<AdminSubmission>,
	db_pool: web::Data<Pool>,
	logger: web::Data<Logger<'_>>,
) -> HttpResponse {
	logger_wrap!(
		logger.info,
		req,
		format!(
			"Admin user registration {{ email: {}, password: {} }}",
			user.email, user.password
		)
	);
	let email_re = Regex::new("^(([^<>()\\[\\].,;:\\s@\"]+(\\.[^<>()\\[\\].,;:\\s@\"]+)*)|(\".+\"))@(([^<>()\\[\\].,;:\\s@\"]+\\.)+[^<>()\\[\\].,;:\\s@\"]{2,})$").unwrap();
	if !email_re.is_match(user.email.to_lowercase().trim()) {
		logger_wrap!(logger.info, req, "Email not valid format");
		return HttpResponse::NotAcceptable().finish()
	}
	let client = match db_pool.get().await {
		Ok(client) => client,
		Err(e) => {
			logger_wrap!(logger.info, req, format!("Client connection error {:?}", e));
			return HttpResponse::InternalServerError().finish()
		}
	};
	match db_new_admin(&client, &user.email, &user.password, &logger, &req).await {
		Either::Left(cookie) => {
			HttpResponse::Ok().insert_header(("content-type", "application/json")).body(cookie.to_string())
		}
		Either::Right(err) => {
			HttpResponse::new(err)
		}
	}
}
