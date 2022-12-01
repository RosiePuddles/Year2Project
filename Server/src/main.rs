#[macro_use] extern crate rocket;
use rocket::http::Status;

mod api;
mod conf;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn base() -> (Status, &'static str) {
    (Status::NotAcceptable, "Please don't do this to me")
}

#[launch]
fn rocket() -> _ {
	let mut custom_conf = rocket::config::Config::default();
	custom_conf.port = conf::PORT;
	rocket::custom(custom_conf)
		.mount("/api", routes![api::submit])
		.mount("/hello", routes![world])
}