#[macro_use]
extern crate rocket;

mod api;
#[doc(hidden)]
mod conf;

#[launch]
pub fn launch() -> _ {
	let mut custom_conf = rocket::config::Config::default();
	custom_conf.port = conf::PORT;
	rocket::custom(custom_conf).mount(
		"/api",
		routes![api::paths::submit, api::paths::new_user, api::paths::login],
	)
}
