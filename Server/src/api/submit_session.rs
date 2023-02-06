use std::fmt::{Display, Formatter};

use actix_web::{web, Error, HttpResponse, ResponseError, post};
use deadpool_postgres::{Client, Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::{Error as PGMError, FromTokioPostgresRow};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error as PGError;
use tokio_postgres::{Row, ToStatement};
use crate::api::db::MyError;

use crate::logger::Logger;

macro_rules! logger_wrap {
    ($t: expr) => {if let Err(e) = $t { println!("Unable to write to log file! {}", e) }};
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct Session {
	pub id: i32,
}

pub async fn db_add_user(client: &Client, session_info: Session, logger: &web::Data<Logger<'_>>) -> Result<(), MyError> {
	let stmt = client.prepare(&include_str!("../../sql/new_session.sql")).await.unwrap();
	
	let rows = match client.query(&stmt, &[&user_info.uname, &user_info.id]).await {
		Ok(rows) => rows,
		Err(e) => {
			logger_wrap!(logger.error(format!("{}:{} {:?}", file!(), line!(), e.to_string())));
			return Err(MyError::ServerError)
		}
	};
	let row = match rows.len() {
		0 => {
			logger_wrap!(logger.error(format!("{}:{} db_add_user expected one row returned from the query. Got 0", file!(), line!())));
			return Err(MyError::ServerError)
		}
		1 => rows.first().unwrap().clone(),
		t => {
			logger_wrap!(logger.warn(format!("{}:{} db_add_user expected one row returned from the query. Got {}", file!(), line!(), t)));
			rows.first().unwrap().clone()
		}
	};
	
	Ok(())
}

impl ResponseError for MyError {
	fn error_response(&self) -> HttpResponse {
		match *self {
			MyError::NotFound => HttpResponse::NotFound().finish(),
			MyError::PoolError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
			_ => HttpResponse::InternalServerError().finish(),
		}
	}
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
