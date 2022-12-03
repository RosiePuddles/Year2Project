//! # API paths
//!
//! This module includes the paths for use with the API

use std::path::Path;

use rocket::{
	error,
	http::{CookieJar, Status},
	info,
	serde::json::Json,
	warn,
};

use crate::{
	api::prelude::{Data, User},
	conf::{API_KEY, DATA_PATH},
};

/// Submit path
///
/// This path is used to submit session data to the server. This required the API key be set via
/// cookies sent with the request. If the given data cannot be serialised into the required struct,
/// this will fail and return a 400 response code (bad request)
///
/// In debug mode this will panic if not using the test user! This is intended for testing!
#[post("/submit", data = "<data>")]
pub fn submit(cookies: &CookieJar<'_>, data: Json<Data>) -> Status {
	println!("{}", data.to_xml());
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY {
			return Status::Forbidden
		}
	} else {
		return Status::Forbidden
	}
	#[cfg(debug_assertions)]
	if data.user_id != "000" {
		panic!("Should only be using test user (test, 1234, 000) in debug mode!")
	}
	let uid_path = Path::new(DATA_PATH).join(Path::new("user_uids.csv"));
	let users_raw = match std::fs::read_to_string(uid_path.clone()) {
		Ok(s) => s,
		Err(e) => {
			error!("Error reading {:?}! {}", uid_path, e);
			return Status::InternalServerError
		}
	};
	if users_raw
		.lines()
		.filter_map(|l| l.split(",").collect::<Vec<_>>().try_into().ok())
		.position(|t: [&str; 3]| t[2] == data.user_id)
		.is_none()
	{
		return Status::Gone
	}
	// #[cfg(not(debug_assertions))]
	let user_path = Path::new(DATA_PATH).join(Path::new(&*data.user_id));
	if !user_path.exists() {
		return Status::InternalServerError
	}
	let session_id = match user_path.read_dir() {
		Ok(i) => match i.count().checked_sub(1) {
			None => {
				error!(
					"User dir {:?} has no files in it! Expected at least 1 file!",
					user_path
				);
				return Status::InternalServerError
			}
			Some(i) => i,
		},
		Err(e) => {
			error!("Could not get DirIter over {:?}!\n{}", user_path, e);
			return Status::InternalServerError
		}
	};
	let user_path = user_path.join(format!("{:0>3x}.xml", session_id));
	if user_path.exists() {
		warn!(
			"Session files in {:?} are incorrectly numbered or session(s) are missing!",
			user_path.parent().unwrap()
		);
		todo!("update session ID to be inline with existing files")
	}
	if let Err(e) = std::fs::write(user_path.clone(), data.to_xml()) {
		error!("Failed to write session data to {:?}! {}", user_path, e);
		return Status::InternalServerError
	}
	Status::Ok
}

/// New user path
///
/// This path is used to create a new user with the given username and pin.
/// If the given data cannot be serialised into the required struct, this will fail and return a 400
/// response code (bad request)
///
/// In debug mode this will not create an actual user! This is intended for use with testing!
// todo: Create new directory for user ID and user XML file in that directory
#[post("/new", data = "<data>")]
pub fn new_user(cookies: &CookieJar<'_>, data: Json<User>) -> (Status, String) {
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY {
			return (Status::Forbidden, "".to_string())
		}
	} else {
		return (Status::Forbidden, "".to_string())
	}
	let uid_path = Path::new(DATA_PATH).join(Path::new("user_uids.csv"));
	let users_raw = match std::fs::read_to_string(uid_path.clone()) {
		Ok(s) => s,
		Err(e) => {
			error!("Error reading {:?}! {}", uid_path, e);
			return (Status::InternalServerError, "".to_string())
		}
	};
	let users = users_raw
		.lines()
		.filter_map(|l| l.split(",").collect::<Vec<_>>().try_into().ok());
	let id_res = users.fold(Ok("000"), |id, curr: [&str; 3]| {
		if id.is_err() {
			id
		} else {
			if curr[0] == data.uname.trim() {
				Err(())
			} else {
				Ok(curr[2])
			}
		}
	});
	let id = match id_res {
		Ok(id_str) => match u32::from_str_radix(id_str, 16) {
			Ok(id) => id + 1,
			Err(e) => {
				info!("Error parsing UID! {}", e);
				return (Status::InternalServerError, "".to_string())
			}
		},
		Err(_) => return (Status::Conflict, "".to_string()),
	};
	#[cfg(not(debug_assertions))]
	match std::fs::OpenOptions::new().append(true).open(uid_path) {
		Ok(f) => match writeln!(f, "{}", format!("{},{},{}", data.uname, data.pin, id)) {
			Ok(_) => {}
			Err(e) => {
				error!("Error appending {:?}! {}", uid_path, e);
				return (Status::InternalServerError, "".to_string())
			}
		},
		Err(e) => {
			error!("Error appending {:?}! {}", uid_path, e);
			return (Status::InternalServerError, "".to_string())
		}
	};
	(Status::Ok, format!("{:0>3x}", id))
}

/// Login path
///
/// This path is used to log in a user with the given username and pin.
/// If the given data cannot be serialised into the required struct, this will fail and return a 400
/// response code (bad request)
///
/// This will panic if not using the test user in debug mode! This is intended for testing purposes!
#[post("/login", data = "<data>")]
pub fn login(cookies: &CookieJar<'_>, data: Json<User>) -> (Status, String) {
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY {
			return (Status::Forbidden, "".to_string())
		}
	} else {
		return (Status::Forbidden, "".to_string())
	}
	#[cfg(debug_assertions)]
	if data.uname != "test" {
		panic!("Should only be using test user (test, 1234, 000) in debug mode!")
	}
	let uid_path = Path::new(DATA_PATH).join(Path::new("user_uids.csv"));
	let users_raw = match std::fs::read_to_string(uid_path.clone()) {
		Ok(s) => s,
		Err(e) => {
			error!("Error reading {:?}!\n{}", uid_path, e);
			return (Status::InternalServerError, "".to_string())
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
	}
}
