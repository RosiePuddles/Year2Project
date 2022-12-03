//! Session tests
use json::object;
use rocket::{
	http::{Cookie, Status},
	local::blocking::Client,
};

use crate::{conf::API_KEY, launch};

/// Submitting sample data. Expecting 200
#[test]
fn submit() {
	let client = Client::tracked(launch()).expect("valid rocket instance");
	let resp = client
		.post(uri!("/api/submit"))
		.cookie(Cookie::new("key", API_KEY))
		.body(
			object! {
				"user_id": "000",
				"time_start": 1671080669,
				"hr_data": [
					{ "time": 1671080702, "pulse": 40 },
					{ "time": 1671080712, "pulse": 35 }
				],
				"gaze_data": [
					{ "time": 1671080702, "yaw": 0, "pitch": -3.9 },
					{ "time": 1671080707, "yaw": 12, "pitch": 55 }
				]
			}
			.to_string(),
		)
		.dispatch();
	assert_eq!(resp.status(), Status::Ok);
}

#[cfg(test)]
mod body {
	//! Body issue tests
	use super::*;

	/// Body missing required data. Expecting 400
	#[test]
	fn submit_bad_body() {
		let client = Client::tracked(launch()).expect("valid rocket instance");
		let resp = client
			.post(uri!("/api/submit"))
			.cookie(Cookie::new("key", API_KEY))
			.body(
				object! {
					"time_start": 1671080669,
					"hr_data": [
						{ "time": 1671080702, "pulse": 40 },
						{ "time": 1671080712, "pulse": 35 }
					],
					"gaze_data": [
						{ "time": 1671080702, "yaw": 0, "pitch": -3.9 },
						{ "time": 1671080707, "yaw": 12, "pitch": 55 }
					]
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
			.post(uri!("/api/submit"))
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
			.post(uri!("/api/submit"))
			.cookie(Cookie::new("key", ""))
			.body(
				object! {
					"user_id": "1",
					"time_start": 1671080669,
					"hr_data": [
						{ "time": 1671080702, "pulse": 40 },
						{ "time": 1671080712, "pulse": 35 }
					],
					"gaze_data": [
						{ "time": 1671080702, "yaw": 0, "pitch": -3.9 },
						{ "time": 1671080707, "yaw": 12, "pitch": 55 }
					]
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
			.post(uri!("/api/submit"))
			.body(
				object! {
					"user_id": "1",
					"time_start": 1671080669,
					"hr_data": [
						{ "time": 1671080702, "pulse": 40 },
						{ "time": 1671080712, "pulse": 35 }
					],
					"gaze_data": [
						{ "time": 1671080702, "yaw": 0, "pitch": -3.9 },
						{ "time": 1671080707, "yaw": 12, "pitch": 55 }
					]
				}
				.to_string(),
			)
			.dispatch();
		assert_eq!(resp.status(), Status::Forbidden);
	}
}
