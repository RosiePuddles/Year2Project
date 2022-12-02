use serde::Deserialize;
use toml;

/// Config struct
///
/// This struct contains the config information for the server
#[derive(Deserialize)]
pub struct Config {
	pub port: Option<u16>,
	pub api_key: String,
	pub data_path: String,
}

fn main() -> Result<(), std::io::Error> {
	let config = match std::fs::read_to_string("config.toml") {
		Ok(s) => match toml::from_str::<Config>(&*s) {
			Ok(c) => c,
			Err(_) => panic!("Unable to parse config file (config.toml)!"),
		},
		Err(_) => panic!("Unable to find config file (config.toml)!"),
	};
	std::fs::write("src/conf.rs", format!(
		"//! # Generated config values\n// @generated\npub const PORT: u16 = {};\npub const API_KEY: &'static str = {:?};\npub const DATA_PATH: &'static str = {:?};",
		config.port.unwrap_or(5000), config.api_key, config.data_path
	))?;
	Ok(())
}
