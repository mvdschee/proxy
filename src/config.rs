use crate::{Error, Result};
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	pub acme: Acme,
	pub routes: Vec<Route>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Acme {
	pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Route {
	pub host: String,
	pub upstream: String,
	#[serde(default)]
	pub tls: RouteTLS,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RouteTLS {
	SelfSigned,
	#[default]
	Acme,
}

impl Config {
	pub fn init() -> Result<Self> {
		let config_path = load_env("CONFIG_PATH")?;
		let config = parse_toml_config(config_path)?;

		Ok(config)
	}
}

fn parse_toml_config(config_path: String) -> Result<Config> {
	let content = fs::read_to_string(config_path)?;
	let config: Config = toml::from_str(&content).map_err(|e| Error::Config(e.to_string()))?;

	Ok(config)
}

fn load_env(key: &str) -> Result<String> {
	match env::var(key) {
		Ok(val) => Ok(val),
		Err(_err) => Err(Error::Env(key.to_string())),
	}
}
