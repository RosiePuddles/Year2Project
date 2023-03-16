//! # API time baby
//!
//! This module provides paths and data structures for any API related routes

use actix_web::{
	dev::{ServiceFactory, ServiceRequest},
	App, Error,
};

use crate::paths;

mod login;
mod new_user;
mod prelude;
mod submit_session;
pub use prelude::Session;

paths!(submit_session::submit_session, login::user_login, new_user::add_user);
