//! # API paths
//!
//! This module includes the paths for use with the API

use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::Request;
use std::num::ParseIntError;

use std::path::Path;

use crate::{
	api::prelude::{Data, User},
	conf::{API_KEY, DATA_PATH},
};

/// Submit path
///
/// This path is used to submit session data to the server. This required the API key be set via
/// cookies sent with the request.
/// If the given data cannot be serialised into the required struct, this will fail and return a 400
/// response code (bad request)
#[post("/submit", data = "<data>")]
pub fn submit(cookies: &CookieJar<'_>, data: Json<Data>) -> Status {
	println!("{}", data.to_xml());
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY {
			return Status::Forbidden;
		}
	} else {
		return Status::Forbidden;
	}
	Status::Ok
}

/// New user path
///
/// This path is used to create a new user with the given username and pin.
/// If the given data cannot be serialised into the required struct, this will fail and return a 400
/// response code (bad request)
#[post("/new", data = "<data>")]
pub fn new_user(cookies: &CookieJar<'_>, data: Json<User>) -> (Status, String) {
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY {
			return (Status::Forbidden, "".to_string());
		}
	} else {
		return (Status::Forbidden, "".to_string());
	}
	let uid_path = Path::new(DATA_PATH).join(Path::new("user_uids.csv"));
	let users_raw = match std::fs::read_to_string(uid_path.clone()) {
		Ok(s) => s,
		Err(e) => {
			println!("Error reading {:?}!\n{}", uid_path, e);
			return (Status::InternalServerError, "".to_string());
		}
	};
	let mut users = users_raw
		.lines()
		.filter_map(|l| l.split(",").collect::<Vec<_>>().try_into().ok());
	let mut users_clone = users.clone();
	if users_clone
		.position(|uname: [&str; 3]| uname[0] == data.uname.trim())
		.is_some()
	{
		return (Status::Conflict, "".to_string());
	}
	let id = match u32::from_str_radix(users.last().unwrap_or(["", "", "000"])[0], 16) {
		Ok(id) => id,
		Err(e) => {
			println!("Error parsing UID!\n{}", e);
			return (Status::InternalServerError, "".to_string());
		}
	};
	#[cfg(not(debug_assertions))]
	match std::fs::OpenOptions::new().append(true).open(uid_path) {
		Ok(f) => match writeln!(f, format!("{},{},{}", data.uname, data.pin, id)) {
			Ok(_) => {}
			Err(e) => {
				println!("Error appending {:?}!\n{}", uid_path, e);
				return (Status::InternalServerError, "".to_string());
			}
		},
		Err(e) => {
			println!("Error appending {:?}!\n{}", uid_path, e);
			return (Status::InternalServerError, "".to_string());
		}
	};
	(Status::Ok, id.to_string())
}

/// Login path
///
/// This path is used to log in a user with the given username and pin.
/// If the given data cannot be serialised into the required struct, this will fail and return a 400
/// response code (bad request)
#[post("/login", data = "<data>")]
pub fn login(cookies: &CookieJar<'_>, data: Json<User>) -> (Status, String) {
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY {
			return (Status::Forbidden, "".to_string());
		}
	} else {
		return (Status::Forbidden, "".to_string());
	}
	let uid_path = Path::new(DATA_PATH).join(Path::new("user_uids.csv"));
	let users_raw = match std::fs::read_to_string(uid_path.clone()) {
		Ok(s) => s,
		Err(e) => {
			println!("Error reading {:?}!\n{}", uid_path, e);
			return (Status::InternalServerError, "".to_string());
		}
	};
	let mut users = users_raw
		.lines()
		.filter_map(|l| l.split(",").collect::<Vec<_>>().try_into().ok());
	return match users.find(|uname: &[&str; 3]| uname[0] == data.uname.trim()) {
		None => (Status::Gone, "".to_string()),
		Some(check) => {
			if data.pin.to_string() == check[1] {
				(Status::Ok, check[2].to_string())
			} else {
				(Status::Unauthorized, "".to_string())
			}
		}
	};
}
