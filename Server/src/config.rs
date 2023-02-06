use serde::{Deserialize, Serialize};

/// Session data
#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
	pub uid: usize,
	pub time: u64,
	pub hr: Vec<HRData>,
	pub gaze: Vec<GazeData>,
}

/// Heartrate data
#[derive(Deserialize, Serialize, Debug)]
pub struct HRData {
	pub time: u64,
	pub pulse: u8,
}

/// Gaze data
#[derive(Deserialize, Serialize, Debug)]
pub struct GazeData {
	pub time: u64,
	pub yaw: i16,
	pub pitch: i16,
}
