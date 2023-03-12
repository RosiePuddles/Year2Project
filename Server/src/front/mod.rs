//! # Frontend data access
//!
//! This module provides paths and data structures to access the frontend with user validation

use actix_web::{
	dev::{ServiceFactory, ServiceRequest},
	App, Error
};

use crate::paths;

mod dash;
mod login;
mod prelude;
mod register;
mod statics;

paths!(
	default statics::not_found_route(),
	login::front_login, login::front_login_post,
	register::front_register, register::front_register_post,
	statics::index, statics::account_term, statics::js_files,
	dash::front_dash
);
