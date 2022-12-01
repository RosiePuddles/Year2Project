use toml;
use serde::Deserialize;

/// Config struct
///
/// This struct contains the config information for the server
#[derive(Deserialize)]
pub struct Config {
	pub port: Option<u16>,
	pub api_key: String,
}

fn main() -> Result<(), std::io::Error> {
	let config = match std::fs::read_to_string("config.toml") {
		Ok(s) => {
			match toml::from_str::<Config>(&*s) {
				Ok(c) => c,
				Err(_) => panic!("Unable to parse config file (config.toml)!")
			}
		},
		Err(_) => panic!("Unable to find config file (config.toml)!")
	};
	std::fs::write("src/conf.rs", format!(
		"pub const PORT: u16 = {};pub const API_KEY: &'static str = {:?};", config.port.unwrap_or(5000), config.api_key
	))?;
	Ok(())
}
