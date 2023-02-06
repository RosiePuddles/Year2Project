use actix_web::{
	dev::{ServiceFactory, ServiceRequest},
	App, Error,
};

mod submit_session;
mod new_user;
pub(crate) mod db;

pub fn paths<T>(mut app: App<T>) -> App<T>
where
	T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>
{
	app = app.service(submit_session::submit_session);
	app.service(new_user::add_user)
}
