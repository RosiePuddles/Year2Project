use actix_web::{web, Error, HttpResponse, post};
use chrono::{DateTime, Local};
use deadpool_postgres::{Client, GenericClient, Pool};
use crate::api::db::MyError;
use crate::api::prelude::submitted::Session;

use crate::logger::Logger;

/// Wrapper around a logger call that prints to stdout if the logger returns an error
macro_rules! logger_wrap {
    ($t: expr) => {if let Err(e) = $t { println!("Unable to write to log file! {}", e) }};
}

pub async fn db_add_user(client: &Client, session: Session, logger: &web::Data<Logger<'_>>) -> Result<(), MyError> {
	macro_rules! get_wrapper {
		($t: expr) => {match $t {
			Ok(t) => t,
			Err(e) => {
				logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
				return Err(MyError::ServerError)
			}
		}};
	}
	// check for existing key and get UUID and end_time
	let stmt = client.prepare(&include_str!("../../sql/submit_session_check.sql")).await.unwrap();
	
	let (uuid, end_time) = match client.query(&stmt, &[&session.key]).await {
		Ok(mut rows) => {
			if let Some(row) = rows.pop() {
				// don't need to check for multiple rows because uname is unique
				(get_wrapper!(row.try_get::<_, i32>(0)), get_wrapper!(row.try_get::<_, DateTime<Local>>(1)))
			} else {
				logger_wrap!(logger.info(format!("{}:{} Session submit with unknown key {}", file!(), line!(), session.key)));
				return Err(MyError::Gone)
			}
		},
		Err(e) => {
			logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
			return Err(MyError::ServerError)
		}
	};
	
	if end_time < Local::now() {
		logger_wrap!(logger.info(format!(
			"{}:{} Submit session with out of date key {} {}",
			file!(), line!(), session.key, end_time.format("%+")
		)));
		return Err(MyError::OutOfDate)
	}
	
	let session = session.into_row(uuid);
	let stmt = client.prepare(&include_str!("../../sql/submit_session.sql")).await.unwrap();
	if let Err(e) = client.query(
		&stmt,
		&[&session.uuid, &session.time, &session.hr, &session.gaze]
	).await {
		logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
		return Err(MyError::ServerError)
	}
	
	Ok(())
}

#[post("/api/submit")]
pub async fn submit_session(
	user: web::Json<Session>, db_pool: web::Data<Pool>, logger: web::Data<Logger<'_>>,
) -> Result<HttpResponse, Error> {
	let user_info: Session = user.into_inner();
	logger_wrap!(logger.info("Connecting to database..."));
	let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
	logger_wrap!(logger.info("Connected to database. Sending query..."));
	
	let new_user = db_add_user(&client, user_info, &logger).await?;
	logger_wrap!(logger.info("Returning"));
	Ok(HttpResponse::Ok().json(new_user))
}
