//! # Frontend data access
//!
//! This module provides the paths for frontend web access, as well as paths for frontend data
//! validation and downloading date

use actix_web::{
	dev::{ServiceFactory, ServiceRequest},
	App, Error,
};

use crate::paths;

mod cleaner;
mod dash;
mod login;
mod prelude;
mod register;
mod statics;
pub use cleaner::clean;

paths!(
	default statics::not_found_route(),
	login::front_login, login::front_login_post,
	register::front_register, register::front_register_post,
	statics::index, statics::js_files,
	dash::front_dash, dash::front_dash_download, dash::download_file
);
