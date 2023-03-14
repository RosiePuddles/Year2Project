use actix_web::{post, web, Error, HttpMessage, HttpRequest, HttpResponse};
use chrono::{DateTime, Local};
use deadpool_postgres::{Client, GenericClient, Pool};
use uuid::Uuid;

use crate::{api::prelude::submitted::Session, db::ApiError, logger::Logger, logger_wrap};

pub async fn db_add_session(
	client: &Client,
	session: Session,
	logger: &web::Data<Logger<'_>>,
	req: &HttpRequest,
) -> Result<(), ApiError> {
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
	// check for existing key and get UUID and end_time
	let stmt = client
		.prepare(&include_str!("../../sql/api/submit_session_check.sql"))
		.await
		.unwrap();

	let (uuid, end_time) = match client.query(&stmt, &[&session.key]).await {
		Ok(mut rows) => {
			if let Some(row) = rows.pop() {
				// don't need to check for multiple rows because uname is unique
				(
					get_wrapper!(row.try_get::<_, Uuid>(0)),
					get_wrapper!(row.try_get::<_, DateTime<Local>>(1)),
				)
			} else {
				logger_wrap!(
					logger.info,
					req,
					format!(
						"{}:{} Session submit with unknown key {}",
						file!(),
						line!(),
						session.key
					)
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

	if end_time < Local::now() {
		logger_wrap!(
			logger.info,
			req,
			format!(
				"{}:{} Submit session with out of date key {} {}",
				file!(),
				line!(),
				session.key,
				end_time.format("%+")
			)
		);
		return Err(ApiError::OutOfDate)
	}

	let session = session.into_row(uuid);
	let stmt = client
		.prepare(&include_str!("../../sql/api/submit_session.sql"))
		.await
		.unwrap();
	if let Err(e) = client
		.query(&stmt, session.query_params().as_slice())
		.await
	{
		logger_wrap!(
			logger.error,
			req,
			format!("{}:{} {:?}", file!(), line!(), e.to_string())
		);
		return Err(ApiError::ServerError)
	}

	Ok(())
}

#[post("/api/submit")]
pub async fn submit_session(
	user: web::Json<Session>,
	db_pool: web::Data<Pool>,
	logger: web::Data<Logger<'_>>,
	req: HttpRequest,
) -> Result<HttpResponse, Error> {
	let session: Session = user.into_inner();
	logger_wrap!(logger.info, req, "Connecting to database...");
	let client: Client = db_pool.get().await.map_err(ApiError::PoolError)?;
	logger_wrap!(logger.info, req, "Connected to database. Sending query...");

	db_add_session(&client, session, &logger, &req).await?;
	logger_wrap!(logger.info, req, "Returning");
	Ok(HttpResponse::Ok().finish())
}
