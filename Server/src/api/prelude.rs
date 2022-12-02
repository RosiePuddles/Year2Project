//! # API data types
//!
//! This module contains the data types used by the API

use rocket::serde::Deserialize;

/// # Submit data struct
///
/// This is deserialized from JSON data submitted with the POST request.
/// An example is given below with some sections omitted. For heart rate data (`hr_data`), see [`HR_Data`]
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
	user_id: String,
	/// Session start time in epoch seconds
	time_start: u64,
	/// Heart rate data vector
	hr_data: Vec<HR_Data>
}

/// # Heart rate data point
#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub struct HR_Data {
	/// Measurement time in epoch seconds
	time: u64,
	/// Pulse value
	pulse: u16
}
