use std::fmt::{Display, Formatter};
use deadpool_postgres::PoolError;
use serde::Deserialize;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Debug)]
pub enum MyError {
	NotFound,
	ServerError,
	PGError(PGError),
	PGMError(PGMError),
	PoolError(PoolError),
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