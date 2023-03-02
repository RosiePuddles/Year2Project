//! # API time baby
//!
//! This module provides paths and data structures for any API related routes

use actix_web::{
	dev::{ServiceFactory, ServiceRequest},
	App, Error,
};

mod db;
mod login;
mod new_user;
mod prelude;
mod submit_session;

pub use db::DbConfig;

/// Add locally defined paths to a given App
pub fn paths<T>(mut app: App<T>) -> App<T>
where
	T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
	app = app.service(submit_session::submit_session);
	app = app.service(login::user_login);
	app.service(new_user::add_user)
}
