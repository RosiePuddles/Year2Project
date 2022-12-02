#[macro_use] extern crate rocket;
use rocket::http::Status;

mod api;
#[doc(hidden)]
mod conf;

#[get("/")]
fn base() -> (Status, &'static str) {
    (Status::NotAcceptable, "Please don't do this to me")
}

#[launch]
pub fn rocket() -> _ {
	let mut custom_conf = rocket::config::Config::default();
	custom_conf.port = conf::PORT;
	rocket::custom(custom_conf)
		.mount("/api", routes![api::paths::submit])
}

#[cfg(test)]
mod test {
	use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, Cookie};
	use super::conf::API_KEY;
	
	#[cfg(test)]
	mod api {
		use super::*;
		
		#[test]
		fn submit() {
			let client = Client::tracked(rocket()).expect("valid rocket instance");
			let mut resp = client.post(uri!("/api/submit"))
				.cookie(Cookie::new("key", API_KEY))
				.body(
					"{'user_id': '1', 'time_start': 1671080669, 'hr_data': []}".replace("'", "\"")
				)
				.dispatch();
			assert_eq!(resp.status(), Status::Ok);
		}
		
		#[test]
		fn submit_bad_body() {
			let client = Client::tracked(rocket()).expect("valid rocket instance");
			let mut resp = client.post(uri!("/api/submit"))
				.cookie(Cookie::new("key", API_KEY))
				.dispatch();
			assert_eq!(resp.status(), Status::BadRequest);
		}
	}
}
