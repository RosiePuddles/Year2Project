use actix_web::{
	dev::{ServiceFactory, ServiceRequest},
	App, Error,
};

mod submit_session;
mod new_user;
pub(crate) mod db;
mod login;
mod prelude;

pub fn paths<T>(mut app: App<T>) -> App<T>
where
	T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>
{
	app = app.service(submit_session::submit_session);
	app = app.service(login::user_login);
	app.service(new_user::add_user)
}
