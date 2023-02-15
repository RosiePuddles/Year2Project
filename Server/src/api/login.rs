use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use actix_web::{web, Error, HttpResponse, post};
use actix_web::http::header::ContentType;
use chrono::Local;
use deadpool_postgres::{Client, Pool};
use json::JsonValue;
use crate::api::db::MyError;
use crate::api::prelude::submitted::User;

use crate::logger::Logger;

macro_rules! logger_wrap {
    ($t: expr) => {if let Err(e) = $t { println!("Unable to write to log file! {}", e) }};
}

pub async fn db_login(client: &Client, uname: String, logger: &web::Data<Logger<'_>>) -> Result<JsonValue, MyError> {
	macro_rules! get_wrapper {
		($t: expr) => {match $t {
			Ok(t) => t,
			Err(e) => {
				logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
				return Err(MyError::ServerError)
			}
		}};
	}
	// check for existing uname and get uuid
	let stmt = client.prepare(&include_str!("../../sql/user_check.sql")).await.unwrap();
	
	let uuid = match client.query(&stmt, &[&uname]).await {
		Ok(mut rows) => {
			if let Some(row) = rows.pop() {
				// don't need to check for multiple rows because uname is unique
				get_wrapper!(row.try_get::<_, i32>(0))
			} else {
				logger_wrap!(logger.info(format!("{}:{} Login with unknown uname requested {:?}", file!(), line!(), uname)));
				return Err(MyError::Gone)
			}
		},
		Err(e) => {
			logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
			return Err(MyError::ServerError)
		}
	};
	
	db_new_otp(client, uuid, uname, logger).await
}

pub(in crate::api) async fn db_new_otp(client: &Client, uuid: i32, uname: String, logger: &web::Data<Logger<'_>>) -> Result<JsonValue, MyError> {
	let stmt = client.prepare(&include_str!("../../sql/user_login.sql")).await.unwrap();
	
	let mut hasher = DefaultHasher::new();
	uuid.hash(&mut hasher);
	uname.hash(&mut hasher);
	Local::now().hash(&mut hasher);
	let otp = format!("{:0>16x}", hasher.finish());
	let end = Local::now() + chrono::Days::new(1);
	
	if let Err(e) = client.query(&stmt, &[&otp, &uuid, &end]).await {
		logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
		return Err(MyError::ServerError)
	};
	
	Ok(json::object!{
		key: otp,
		time: end.format("%+").to_string()
	})
}

#[post("/api/login")]
pub async fn user_login(
	user: web::Json<User>, db_pool: web::Data<Pool>, logger: web::Data<Logger<'_>>,
) -> Result<HttpResponse, Error> {
	let user = user.into_inner();
	logger_wrap!(logger.info("Connecting to database..."));
	let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
	logger_wrap!(logger.info("Connected to database. Sending query..."));
	
	let ret = db_login(&client, user.uname, &logger).await?;
	logger_wrap!(logger.info("Returning"));
	Ok(HttpResponse::Ok().content_type(ContentType::json()).body(ret.to_string()))
}
