//! Session tests
use json::object;
use rocket::{
	http::{Cookie, Status},
	local::blocking::Client,
};

use crate::{conf::API_KEY, launch};

/// Make new user. Expecting 200
#[test]
fn new() {
	let client = Client::tracked(launch()).expect("valid rocket instance");
	let resp = client
		.post(uri!("/api/login"))
		.cookie(Cookie::new("key", API_KEY))
		.body(
			object! {
				"uname": "test",
				"pin": 1234
			}
			.to_string(),
		)
		.dispatch();
	assert_eq!(resp.status(), Status::Ok);
	assert_eq!(resp.into_string(), Some("000".to_string()));
}

/// Login attempt with the wrong pin. Expecting 401
#[test]
fn wrong_pin() {
	let client = Client::tracked(launch()).expect("valid rocket instance");
	let resp = client
		.post(uri!("/api/login"))
		.cookie(Cookie::new("key", API_KEY))
		.body(
			object! {
				"uname": "test",
				"pin": 0000
			}
			.to_string(),
		)
		.dispatch();
	assert_eq!(resp.status(), Status::Unauthorized);
}

#[cfg(test)]
mod body {
	//! Body issue tests
	use super::*;

	/// Body missing required data. Expecting 422
	#[test]
	fn submit_bad_body() {
		let client = Client::tracked(launch()).expect("valid rocket instance");
		let resp = client
			.post(uri!("/api/login"))
			.cookie(Cookie::new("key", API_KEY))
			.body(
				object! {
					"uname": "test",
				}
				.to_string(),
			)
			.dispatch();
		assert_eq!(resp.status(), Status::UnprocessableEntity);
	}

	/// No body. Expecting 400
	#[test]
	fn submit_no_body() {
		let client = Client::tracked(launch()).expect("valid rocket instance");
		let resp = client
			.post(uri!("/api/login"))
			.cookie(Cookie::new("key", API_KEY))
			.dispatch();
		assert_eq!(resp.status(), Status::BadRequest);
	}
}

#[cfg(test)]
mod api_key {
	//! API key issues
	use super::*;

	/// Incorrect API key. Expecting 403
	#[test]
	fn submit_bad_key() {
		let client = Client::tracked(launch()).expect("valid rocket instance");
		let resp = client
			.post(uri!("/api/login"))
			.cookie(Cookie::new("key", ""))
			.body(
				object! {
					"uname": "test",
					"pin": 1234
				}
				.to_string(),
			)
			.dispatch();
		assert_eq!(resp.status(), Status::Forbidden);
	}

	/// Noi API key. Expecting 403
	#[test]
	fn submit_no_key() {
		let client = Client::tracked(launch()).expect("valid rocket instance");
		let resp = client
			.post(uri!("/api/login"))
			.body(
				object! {
					"uname": "test",
					"pin": 1234
				}
				.to_string(),
			)
			.dispatch();
		assert_eq!(resp.status(), Status::Forbidden);
	}
}
