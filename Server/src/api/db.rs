use std::fmt::{Display, Formatter};
use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use serde::Deserialize;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Debug)]
pub enum MyError {
	ServerError,
	Conflict,
	Gone,
	OutOfDate,
	PGError(PGError),
	PGMError(PGMError),
	PoolError(PoolError),
}

impl ResponseError for MyError {
	fn error_response(&self) -> HttpResponse {
		match *self {
			MyError::PoolError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
			MyError::Conflict => HttpResponse::Conflict().finish(),
			MyError::Gone => HttpResponse::Gone().finish(),
			MyError::OutOfDate => HttpResponse::Unauthorized().finish(),
			_ => HttpResponse::InternalServerError().finish(),
		}
	}
}

#[derive(Debug, Default, Deserialize)]
pub struct ExampleConfig {
	pub server_addr: String,
	pub pg: deadpool_postgres::Config,
}

impl Display for MyError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl From<PGError> for MyError {
	fn from(value: PGError) -> Self { Self::PGError(value) }
}

impl From<PGMError> for MyError {
	fn from(value: PGMError) -> Self { Self::PGMError(value) }
}

impl From<PoolError> for MyError {
	fn from(value: PoolError) -> Self { Self::PoolError(value) }
}

impl std::error::Error for MyError {}