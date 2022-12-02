#[macro_use]
extern crate rocket;
use rocket::http::Status;

mod api;
#[doc(hidden)]
mod conf;

#[get("/")]
fn base() -> (Status, &'static str) { (Status::NotAcceptable, "Please don't do this to me") }

#[launch]
pub fn rocket() -> _ {
	let mut custom_conf = rocket::config::Config::default();
	custom_conf.port = conf::PORT;
	rocket::custom(custom_conf).mount(
		"/api",
		routes![api::paths::submit, api::paths::new_user, api::paths::login],
	)
}

#[cfg(test)]
mod test {
	use super::conf::API_KEY;
	use super::rocket;
	use rocket::http::{Cookie, Status};
	use rocket::local::blocking::Client;

	#[cfg(test)]
	mod api {
		use super::*;
		use json::object;

		#[test]
		fn submit() {
			let client = Client::tracked(rocket()).expect("valid rocket instance");
			let mut resp = client
				.post(uri!("/api/submit"))
				.cookie(Cookie::new("key", API_KEY))
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
			assert_eq!(resp.status(), Status::Ok);
		}

		#[test]
		fn submit_bad_body() {
			let client = Client::tracked(rocket()).expect("valid rocket instance");
			let mut resp = client
				.post(uri!("/api/submit"))
				.cookie(Cookie::new("key", API_KEY))
				.dispatch();
			assert_eq!(resp.status(), Status::BadRequest);
		}
	}
}
