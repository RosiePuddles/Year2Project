//! # Login path `/api/login`
//!
//! This handles user logins and locally exposes [`db_new_user_key`] to allow a new user to return an user key on
//! success

use std::{
	collections::{HashMap, HashSet},
	io::Write,
};

use actix_files::NamedFile;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use chrono::Local;
use deadpool_postgres::Pool;
use rand::prelude::SliceRandom;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
	api::Session,
	front::prelude::{DateRange, DownloadFormat},
	logger::Logger,
	logger_wrap,
};

async fn cookie_check(
	req: &HttpRequest,
	db_pool: &web::Data<Pool>,
	logger: &web::Data<Logger<'_>>,
) -> Result<i32, HttpResponse> {
	if let Some(cookie) = req.cookie("login") {
		match db_pool.get().await {
			Ok(client) => {
				let stmt = client
					.prepare(include_str!("../../sql/admin/auth_key.sql"))
					.await
					.unwrap();
				match client.query(&stmt, &[&cookie.value()]).await {
					Ok(mut rows) => {
						if let Some(uuid_row) = rows.pop() {
							match uuid_row.try_get::<_, i32>(0) {
								Ok(uuid) => return Ok(uuid),
								Err(e) => {
									logger_wrap!(logger.error, req, format!("UUID get unwrap error - {:?}", e))
								}
							}
						}
					}
					Err(e) => {
						logger_wrap!(logger.error, req, format!("Cookie check query error - {:?}", e))
					}
				}
			}
			Err(e) => {
				logger_wrap!(
					logger.error,
					req,
					format!("Error connecting to db on dashboard cookie check - {:?}", e)
				)
			}
		}
	}
	Err(HttpResponse::Found().insert_header(("Location", "/login")).finish())
}

/// User login path
#[get("/dashboard")]
pub async fn front_dash(req: HttpRequest, db_pool: web::Data<Pool>, logger: web::Data<Logger<'_>>) -> HttpResponse {
	match cookie_check(&req, &db_pool, &logger).await {
		Ok(_) => NamedFile::open("front/dash.html")
			.expect("Could not find dashboard file (dash.html)")
			.into_response(&req),
		Err(redirect) => redirect,
	}
}

/// User login path
#[post("/dashboard")]
pub async fn front_dash_download(
	req: HttpRequest,
	dates: web::Json<DateRange>,
	db_pool: web::Data<Pool>,
	logger: web::Data<Logger<'_>>,
) -> HttpResponse {
	let uuid = match cookie_check(&req, &db_pool, &logger).await {
		Ok(uuid) => uuid,
		Err(redirect) => return redirect,
	};
	let client = match db_pool.get().await {
		Ok(client) => client,
		Err(err) => {
			logger_wrap!(
				logger.error,
				req,
				format!("Error connecting to db on login cookie check - {:?}", err)
			);
			return HttpResponse::InternalServerError().finish()
		}
	};
	let stmt = client
		.prepare(include_str!("../../sql/admin/download.sql"))
		.await
		.unwrap();
	let data = match client.query(&stmt, &[&dates.start, &dates.end]).await {
		Ok(rows) => {
			let mut sessions = Vec::new();
			for row in rows {
				sessions.push(Session::from_row(row).unwrap())
			}
			let unique_uuids = sessions.iter().map(|s| s.uuid).collect::<HashSet<_>>();
			let mut new_uuids = unique_uuids.iter().collect::<Vec<_>>();
			new_uuids.shuffle(&mut rand::thread_rng());
			let uuid_map = unique_uuids
				.iter()
				.zip(new_uuids)
				.map(|(original, map)| (original.clone(), map.clone()))
				.collect::<HashMap<_, _>>();
			let mut mapped_session = Vec::new();
			match dates.format {
				DownloadFormat::JSON => {
					for mut session in sessions {
						session.uuid = uuid_map
							.get(&session.uuid)
							.expect("UUID map didn't work correctly")
							.clone();
						mapped_session.push(serde_json::to_string(&session).unwrap())
					}
					format!("[{}]", mapped_session.join(","))
				}
				DownloadFormat::CSV => {
					mapped_session.push("uuid,time,hr,meditation,gaze".to_string());
					for mut session in sessions {
						session.uuid = uuid_map
							.get(&session.uuid)
							.expect("UUID map didn't work correctly")
							.clone();
						mapped_session.push(format!(
							"{},{},[{}],[{}],[{}]",
							session.uuid,
							session.time.to_rfc3339(),
							session.hr.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(","),
							session
								.meditation
								.iter()
								.map(|t| t.to_string())
								.collect::<Vec<_>>()
								.join(","),
							session
								.gaze
								.iter()
								.map(|t| format!("[{},{}]", t.x(), t.y()))
								.collect::<Vec<_>>()
								.join(",")
						))
					}
					mapped_session.join("\n")
				}
			}
		}
		Err(err) => {
			logger_wrap!(logger.error, req, format!("Download query request error - {:?}", err));
			return HttpResponse::InternalServerError().finish()
		}
	};
	let file_extension = match dates.format {
		DownloadFormat::JSON => "json",
		DownloadFormat::CSV => "csv",
	};
	let f_name = format!("{}-{}.{}", uuid, Local::now().to_rfc3339(), file_extension);
	if !std::path::Path::new("tmp").exists() {
		std::fs::create_dir("tmp").expect("Unable to create tmp dir");
	}
	let mut file = match std::fs::OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(std::env::current_dir().unwrap().join("tmp").join(f_name.clone()))
	{
		Ok(f) => f,
		Err(e) => {
			logger_wrap!(logger.error, req, format!("File creation error - {:?}", e));
			return HttpResponse::InternalServerError().finish()
		}
	};
	if let Err(e) = file.write(data.as_bytes()) {
		logger_wrap!(logger.error, req, format!("File write error - {:?}", e));
		return HttpResponse::InternalServerError().finish()
	}
	HttpResponse::Ok().content_type("application/json").body(
		json::object! {
			redirect: format!("download/{}", &f_name),
			name: f_name
		}
		.to_string(),
	)
}

#[get("/download/{path}")]
pub async fn download_file(
	req: HttpRequest,
	path: web::Path<(String,)>,
	db_pool: web::Data<Pool>,
	logger: web::Data<Logger<'_>>,
) -> HttpResponse {
	let path = path.into_inner();
	match cookie_check(&req, &db_pool, &logger).await {
		Ok(_) => {}
		Err(_) => {
			logger_wrap!(logger.info, req, "Request with no login cookie");
			return HttpResponse::BadRequest().finish()
		}
	}
	let f_path = std::path::Path::new("tmp").join(path.0);
	match NamedFile::open(f_path.clone()) {
		Ok(nf) => nf.into_response(&req),
		Err(e) => {
			logger_wrap!(logger.info, req, format!("Download file error - {}", e));
			if !f_path.exists() {
				HttpResponse::Gone().finish()
			} else {
				HttpResponse::NotFound().finish()
			}
		}
	}
}
