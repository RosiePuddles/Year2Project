//! General structs to represent tables
//!
//! Types are from the impls for the [postgres_types::FromSql](https://docs.rs/postgres-types/0.2.4/postgres_types/trait.FromSql.html#types) docs
use chrono::{DateTime, Local};
use geo_types::Point;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

/// Internal user serialised from the `users` table
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
	pub uname: String,
	pub uuid: Uuid,
}

/// Internal OTP serialised from the `keys` table
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "keys")]
pub struct OTP {
	pub key: String,
	pub uuid: Uuid,
	pub end_time: DateTime<Local>,
}

/// Internal session serialised from the `sessions` table
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "sessions")]
pub struct Session {
	pub uuid: Uuid,
	pub time: DateTime<Local>,
	pub hr: Vec<i32>,
	pub meditation: Vec<i32>,
	pub gaze: Vec<Point>,
}

pub mod submitted {
	//! Submitted data types
	//!
	//! These are the types submitted and not representations of db tables
	use uuid::Uuid;

	use super::*;

	/// Required data to log in and generate a new OTP
	#[derive(Deserialize, Serialize)]
	pub struct User {
		pub uname: String,
	}

	/// Submitted session data
	#[derive(Deserialize, Serialize)]
	pub struct Session {
		pub key: String,
		pub time: DateTime<Local>,
		pub hr: Vec<i32>,
		pub meditation: Vec<i32>,
		pub gaze: Vec<Point>,
	}

	impl Session {
		/// Turn a `Self` into a [`Session`][super::session]
		pub fn into_row(self, uuid: Uuid) -> super::Session {
			super::Session {
				uuid,
				time: self.time,
				hr: self.hr,
				meditation: self.meditation,
				gaze: self.gaze,
			}
		}
	}
}
