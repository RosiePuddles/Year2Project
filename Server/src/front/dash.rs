//! # Login path `/api/login`
//!
//! This handles user logins and locally exposes [`db_new_user_key`] to allow a new user to return an user key on
//! success

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};
use actix_files::NamedFile;

use actix_web::{get, http::header::ContentType, post, web, Error, HttpMessage, HttpRequest, HttpResponse};
use chrono::Local;
use deadpool_postgres::{Client, Pool};
use crate::logger::Logger;
use crate::logger_wrap;

/// User login path
#[get("/dashboard")]
pub async fn front_dash(req: HttpRequest, db_pool: web::Data<Pool>, logger: web::Data<Logger<'_>>) -> HttpResponse {
	// check for valid login cookie
	if let Some(cookie) = req.cookie("login") {
		match db_pool.get().await {
			Ok(client) => {
				let stmt = client.prepare(include_str!("../../sql/admin/auth_key.sql")).await.unwrap();
				match client.query(&stmt, &[&cookie.value()]).await {
					Ok(rows) => {
						if !rows.is_empty() {
							return NamedFile::open("front/dash.html").expect("Could not find dashboard file (dash.html)").into_response(&req)
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
	HttpResponse::Found().insert_header(("Location", "/login")).finish()
}
