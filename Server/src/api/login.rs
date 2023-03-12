//! # Login path `/api/login`
//!
//! This handles user logins and locally exposes [`db_new_user_key`] to allow a new user to return an user key on
//! success

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

use actix_web::{http::header::ContentType, post, web, Error, HttpMessage, HttpRequest, HttpResponse};
use chrono::Local;
use deadpool_postgres::{Client, Pool};
use json::JsonValue;
use rand::SeedableRng;
use rand_chacha::rand_core::block::BlockRngCore;

use crate::{api::prelude::submitted::User, db::ApiError, logger::Logger, logger_wrap};

/// Verifies a user login and returns a user key from [`db_new_user_key`]
async fn db_login(
	client: &Client,
	uname: String,
	logger: &web::Data<Logger<'_>>,
	req: &HttpRequest,
) -> Result<JsonValue, ApiError> {
	macro_rules! get_wrapper {
		($t: expr) => {
			match $t {
				Ok(t) => t,
				Err(e) => {
					logger_wrap!(
						logger.error,
						req,
						format!("{}:{} {:?}", file!(), line!(), e.to_string())
					);
					return Err(ApiError::ServerError)
				}
			}
		};
	}
	// check for existing uname and get uuid
	let stmt = client
		.prepare(&include_str!("../../sql/api/user_check.sql"))
		.await
		.unwrap();

	let uuid = match client.query(&stmt, &[&uname]).await {
		Ok(mut rows) => {
			if let Some(row) = rows.pop() {
				// don't need to check for multiple rows because uname is unique
				get_wrapper!(row.try_get::<_, i32>(0))
			} else {
				logger_wrap!(
					logger.info,
					req,
					format!("{}:{} Login with unknown uname requested {:?}", file!(), line!(), uname)
				);
				return Err(ApiError::Gone)
			}
		}
		Err(e) => {
			logger_wrap!(
				logger.error,
				req,
				format!("{}:{} {:?}", file!(), line!(), e.to_string())
			);
			return Err(ApiError::ServerError)
		}
	};

	db_new_user_key(client, uuid, uname, logger, req).await
}

/// Get a new user key from a given UUID and uname
pub(in crate::api) async fn db_new_user_key(
	client: &Client,
	uuid: i32,
	uname: String,
	logger: &web::Data<Logger<'_>>,
	req: &HttpRequest,
) -> Result<JsonValue, ApiError> {
	let stmt = client
		.prepare(&include_str!("../../sql/api/user_login.sql"))
		.await
		.unwrap();

	let mut hasher = DefaultHasher::new();
	uuid.hash(&mut hasher);
	uname.hash(&mut hasher);
	Local::now().hash(&mut hasher);
	let mut rng = rand_chacha::ChaCha20Core::seed_from_u64(hasher.finish());
	let mut r = Default::default();
	rng.generate(&mut r);
	let user_key = r
		.as_ref()
		.iter()
		.fold(String::new(), |acc, t| format!("{}{:08x}", acc, t));
	let end = Local::now() + chrono::Days::new(1);

	if let Err(e) = client.query(&stmt, &[&user_key, &uuid, &end]).await {
		logger_wrap!(
			logger.error,
			req,
			format!("{}:{} {:?}", file!(), line!(), e.to_string())
		);
		return Err(ApiError::ServerError)
	};

	Ok(json::object! {
		key: user_key,
		time: end.format("%+").to_string()
	})
}

/// User login path
#[post("/api/login")]
pub async fn user_login(
	req: HttpRequest,
	user: web::Json<User>,
	db_pool: web::Data<Pool>,
	logger: web::Data<Logger<'_>>,
) -> Result<HttpResponse, Error> {
	let user = user.into_inner();
	logger_wrap!(logger.info, req, "Connecting to database...");
	let client: Client = db_pool.get().await.map_err(ApiError::PoolError)?;
	logger_wrap!(logger.info, req, "Connected to database. Sending query...");

	let ret = db_login(&client, user.uname, &logger, &req).await?;
	logger_wrap!(logger.info, req, "Returning");
	Ok(HttpResponse::Ok()
		.content_type(ContentType::json())
		.body(ret.to_string()))
}
