//! # API functions
//!
//! This module includes paths for the API used with the Oculus

use rocket::http::{Status, CookieJar};
use rocket::Request;

use crate::conf::API_KEY;

/// Submit path
///
/// This path is used to submit session data to the server. This required the API key be set via
/// cookies sent with the request
#[post("/submit")]
pub fn submit(cookies: &CookieJar<'_>) -> Status {
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY { return Status::Forbidden }
	} else {
		return Status::Forbidden
	}
	Status::Ok
}
