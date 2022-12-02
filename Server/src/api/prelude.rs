//! # API data types
//!
//! This module contains the data types used by the API

use rocket::serde::Deserialize;
use serde::Serialize;

/// # Submit data struct
///
/// This is deserialized from JSON data submitted with the POST request.
/// An example is given below with some sections omitted. For heart rate data (`hr_data`), see
/// [`HR_Data`]
/// ```json
/// {
/// 	"user_id": "UID",
/// 	"time_start": 1669980122,
/// 	"hr_data": [ ... ]
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct Data {
	/// Unique user ID
	pub user_id: String,
	/// Session start time in epoch seconds
	pub time_start: u64,
	/// Heart rate data-points
	pub hr_data: Vec<HR_Data>,
	/// Gaze data-points
	pub gaze_data: Vec<GazeData>,
}

/// # Heart rate data point
#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub struct HR_Data {
	/// Measurement time in epoch seconds
	pub time: u64,
	/// Pulse value
	pub pulse: u16,
}

/// # Gaze data point
#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub struct GazeData {
	/// Measurement time in epoch seconds
	pub time: u64,
	/// Yaw rotation (left right)
	pub yaw: f32,
	/// Pitch rotation (up down)
	pub pitch: f32,
}

/// # Internal user representation
///
/// Used to represent a user internally. Used for locating users or checking if a new one can be
/// made
#[derive(Deserialize, Debug)]
pub struct User {
	/// Username
	pub uname: String,
	/// Login pin
	pub pin: u16,
}
