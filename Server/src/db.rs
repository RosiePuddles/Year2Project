use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use serde::Deserialize;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

/// API error
///
/// This is the general type returned if an error occurs in an API path
#[derive(Debug)]
pub enum ApiError {
	ServerError,
	Conflict,
	Gone,
	OutOfDate,
	PGError(PGError),
	PGMError(PGMError),
	PoolError(PoolError),
}

impl ResponseError for ApiError {
	fn error_response(&self) -> HttpResponse {
		match *self {
			ApiError::PoolError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
			ApiError::Conflict => HttpResponse::Conflict().finish(),
			ApiError::Gone => HttpResponse::Gone().finish(),
			ApiError::OutOfDate => HttpResponse::Unauthorized().finish(),
			_ => HttpResponse::InternalServerError().finish(),
		}
	}
}

/// Database config. I don't really know what it means sorry. Go look at the Actix db docs
#[derive(Debug, Default, Deserialize)]
pub struct DbConfig {
	pub server_addr: String,
	pub pg: deadpool_postgres::Config,
}

impl Display for ApiError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl From<PGError> for ApiError {
	fn from(value: PGError) -> Self { Self::PGError(value) }
}

impl From<PGMError> for ApiError {
	fn from(value: PGMError) -> Self { Self::PGMError(value) }
}

impl From<PoolError> for ApiError {
	fn from(value: PoolError) -> Self { Self::PoolError(value) }
}

impl std::error::Error for ApiError {}
