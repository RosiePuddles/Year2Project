//! # API paths
//!
//! This module includes the paths for use with the API

use rocket::http::{Status, CookieJar};
use rocket::serde::json::Json;
use rocket::Request;

use crate::{conf::API_KEY, api::prelude::Data};

/// Submit path
///
/// This path is used to submit session data to the server. This required the API key be set via
/// cookies sent with the request.
/// If the given data cannot be serialised into the required struct, this will fail and return a 400
/// response code (bad request)
#[post("/submit", data="<data>")]
pub fn submit(cookies: &CookieJar<'_>, data: Json<Data>) -> Status {
	println!("{:?}", data);
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY { return Status::Forbidden }
	} else {
		return Status::Forbidden
	}
	Status::Ok
}