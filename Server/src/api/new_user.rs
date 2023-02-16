//! # New user path `/api/new`
//!
//! Path for adding a new user

use actix_web::{http::header::ContentType, post, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use json::JsonValue;

use crate::{
	api::{db::ApiError, prelude::submitted::User},
	logger::Logger,
};

macro_rules! logger_wrap {
	($t: expr) => {
		if let Err(e) = $t {
			println!("Unable to write to log file! {}", e)
		}
	};
}

/// Adds a new user to the user table and returns a user key from [`db_new_otp`][crate::api::login::db_new_otp]
async fn db_add_user(client: &Client, uname: String, logger: &web::Data<Logger<'_>>) -> Result<JsonValue, ApiError> {
	macro_rules! get_wrapper {
		($t: expr) => {
			match $t {
				Ok(t) => t,
				Err(e) => {
					logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
					return Err(ApiError::ServerError)
				}
			}
		};
	}
	// check for conflicting uname
	let stmt = client.prepare(&include_str!("../../sql/user_check.sql")).await.unwrap();

	match client.query(&stmt, &[&uname]).await {
		Ok(rows) => {
			if !rows.is_empty() {
				logger_wrap!(logger.info(format!(
					"{}:{} Conflicting new uname requested {:?}",
					file!(),
					line!(),
					uname
				)));
				return Err(ApiError::Conflict)
			}
		}
		Err(e) => {
			logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
			return Err(ApiError::ServerError)
		}
	};

	let stmt = client.prepare(&include_str!("../../sql/new_user.sql")).await.unwrap();

	let rows = get_wrapper!(client.query(&stmt, &[&uname]).await);
	let uuid = match rows.len() {
		0 => {
			logger_wrap!(logger.error(format!(
				"{}:{} db_login expected one row returned from the query. Got 0",
				file!(),
				line!()
			)));
			return Err(ApiError::ServerError)
		}
		1 => get_wrapper!(rows.first().unwrap().try_get::<_, i32>(0)),
		t => {
			logger_wrap!(logger.warn(format!(
				"{}:{} db_login expected one row returned from the query. Got {}",
				file!(),
				line!(),
				t
			)));
			get_wrapper!(rows.first().unwrap().try_get::<_, i32>(0))
		}
	};

	crate::api::login::db_new_user_key(client, uuid, uname, logger).await
}

/// App path
#[post("/api/new")]
pub async fn add_user(
	user: web::Json<User>, db_pool: web::Data<Pool>, logger: web::Data<Logger<'_>>,
) -> Result<HttpResponse, Error> {
	let user_info: User = user.into_inner();
	logger_wrap!(logger.info("Connecting to database..."));
	let client: Client = db_pool.get().await.map_err(ApiError::PoolError)?;
	logger_wrap!(logger.info("Connected to database. Sending query..."));

	let ret = db_add_user(&client, user_info.uname, &logger).await?;
	logger_wrap!(logger.info("Returning"));
	Ok(HttpResponse::Ok()
		.content_type(ContentType::json())
		.body(ret.to_string()))
}
