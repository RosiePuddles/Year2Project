use crate::api::prelude::{Data, GazeData, HR_Data};

impl Data {
	/// Converts data into the required XML representation
	pub fn to_xml(&self) -> String {
		format!(
			"<?xml version=\"1.0\" encoding=\"UTF-8\" ?><!DOCTYPE session SYSTEM \"../dtds/session.dtd\"><session id=\"{}\"><time>{}</time><hr_data>{}</hr_data><gaze_data>{}</gaze_data></session>",
			0, self.time_start, self.hr_data.iter().fold(String::new(), |acc, d| format!("{}{}", acc, d.to_xml())),
			self.gaze_data.iter().fold(String::new(), |acc, d| format!("{}{}", acc, d.to_xml()))
		)
	}
}

impl HR_Data {
	/// Converts data into the required XML representation
	pub fn to_xml(&self) -> String {
		format!(
			"<hr_element><time>{}</time><pulse>{}</pulse></hr_element>",
			self.time, self.pulse
		)
	}
}

impl GazeData {
	/// Converts data into the required XML representation
	pub fn to_xml(&self) -> String {
		format!(
			"<gaze_element><time>{}</time><yaw>{}</yaw><pitch>{}</pitch></gaze_element>",
			self.time, self.yaw, self.pitch
		)
	}
}
