//! # API paths
//!
//! This module includes the paths for use with the API

use std::io::Write;
use std::path::Path;

use rocket::{
	http::{CookieJar, Status},
	serde::json::Json,
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
	debug!("{:?}", data);
	// Check for correct API key
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY {
			return Status::Forbidden
		}
	} else {
		return Status::Forbidden
	}
	// Test user protection
	#[cfg(test)]
	if data.user_id != "000" {
		panic!("Should only be using test user (test, 1234, 000) in debug mode!")
	}
	// Get user csv data path and read into users_raw
	let uid_path = Path::new(DATA_PATH).join("user_uids.csv");
	let users_raw = match std::fs::read_to_string(uid_path.clone()) {
		Ok(s) => s,
		Err(e) => {
			error!("Error reading {:?}! {}", uid_path, e);
			return Status::InternalServerError
		}
	};
	// Check that the given user ID exists
	if users_raw
		.lines()
		.filter_map(|l| l.split(",").collect::<Vec<_>>().try_into().ok())
		.position(|t: [&str; 3]| t[2] == data.user_id)
		.is_none()
	{
		return Status::Gone
	}
	{
		// Get user folder and check it exists
		let user_path = Path::new(DATA_PATH).join(&*data.user_id);
		if !user_path.exists() {
			error!("User data folder {:?} does not exist!", user_path);
			return Status::InternalServerError
		}
		// Getnew session ID by counting files in directory
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
		// Write session XML data to new session file
		let mut session_path = user_path.join(format!("{:0>3x}.xml", session_id));
		if session_path.exists() {
			warn!(
				"Session files in {:?} are incorrectly numbered or session(s) are missing!",
				session_path.parent().unwrap()
			);
			// Get next largest session ID
			let dir_iter = match user_path.read_dir() {
				Ok(iter) => iter,
				Err(e) => {
					error!("Could not get DirIter over {:?}!\n{}", user_path, e);
					return Status::InternalServerError
				}
			};
			let session_id = dir_iter
				.filter_map(|t| t.ok())
				.map(|t| t.file_name())
				.filter_map(|t| usize::from_str_radix(t.to_str().unwrap(), 16).ok())
				.max()
				.unwrap_or(0);
			session_path = user_path.join(format!("{:0>3x}.xml", session_id));
		}
		// Write session data to file
		if let Err(e) = std::fs::write(session_path.clone(), data.to_xml()) {
			error!("Failed to write session data to {:?}! {}", session_path, e);
			return Status::InternalServerError
		}
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
#[post("/new", data = "<data>")]
pub fn new_user(cookies: &CookieJar<'_>, data: Json<User>) -> (Status, String) {
	debug!("{:?}", data);
	// Check for correct API key
	if let Some(c) = cookies.get("key") {
		if c.value() != API_KEY {
			return (Status::Forbidden, "".to_string())
		}
	} else {
		return (Status::Forbidden, "".to_string())
	}
	#[cfg(not(test))]
	if data.uname != "test2" {
		panic!("New user test should only use the uname test2!")
	}
	// Get user uid csv path
	let uid_path = Path::new(DATA_PATH).join("user_uids.csv");
	let users_raw = match std::fs::read_to_string(uid_path.clone()) {
		Ok(s) => s,
		Err(e) => {
			error!("{} | Error reading {:?}! {}", line!(), uid_path, e);
			return (Status::InternalServerError, "".to_string())
		}
	};
	let users = users_raw
		.lines()
		.filter_map(|l| l.split(",").collect::<Vec<_>>().try_into().ok());
	let id_res = users.fold(Ok(0), |id, curr: [&str; 3]| {
		if let Ok(previous) = id {
			if curr[0] == data.uname.trim() {
				return Err(())
			}
			if let Ok(parse) = u32::from_str_radix(curr[2], 16) {
				Ok(parse.max(previous))
			} else { Ok(previous) }
		} else {
			Err(())
		}
	});
	let id = match id_res {
		Ok(id) => id + 1,
		Err(_) => return (Status::Conflict, "".to_string()),
	};
	// Write new user to user uid csv file
	match std::fs::OpenOptions::new().append(true).open(uid_path.clone()) {
		Ok(mut f) => match f.write_all(format!("\n{},{},{:0>3x}", data.uname, data.pin, id).as_bytes()) {
			Ok(_) => {}
			Err(e) => {
				error!("{} | Error appending {:?}! {}", line!(), uid_path, e);
				return (Status::InternalServerError, "".to_string())
			}
		},
		Err(e) => {
			error!("{} | Error appending {:?}! {}", line!(), uid_path, e);
			return (Status::InternalServerError, "".to_string())
		}
	};
	// Make new user file
	let user_dir = Path::new(DATA_PATH).join(format!("{:0>3x}", id));
	match std::fs::create_dir(user_dir.clone()) {
		Ok(_) => {}
		Err(e) => {
			error!("{} | Error creating user dir {:?}! {}", line!(), user_dir, e);
			return (Status::InternalServerError, "".to_string())
		}
	}
	match std::fs::write(user_dir.join("info.xml"), format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\" ?><!DOCTYPE user SYSTEM \"../dtds/user.dtd\"><user><name>{}</name><session_data></session_data><pdata></pdata></user>",
		data.uname
	)) {
		Ok(_) => {}
		Err(e) => {
			error!("{} | Error creating user folder {:?}! {}", line!(), user_dir.join("info.xml"), e);
			return (Status::InternalServerError, "".to_string())
		}
	}
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
	debug!("{:?}", data);
	// Check for correct API key
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
	// Get user UID csv file
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
	// Check if given username exists with matching pin and return appropriate response code
	match users.find(|uname: &[&str; 3]| uname[0] == data.uname.trim()) {
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
