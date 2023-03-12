use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

/// Internal user serialised from the `admins` table. This represents someone with a server based
/// account allowing for downloading data
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "admins")]
pub struct Admin {
	pub email: String,
	pub uuid: i32,
	pub pwdh: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminSubmission {
	pub email: String,
	pub password: String,
}

/// Admin auth key row for the table `admin_auth`
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "admin_auth")]
pub struct AdminAuth {
	pub auth_key: String,
	pub uuid: i32,
	pub end_time: DateTime<Local>,
}
