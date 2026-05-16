use crate::{
	Error, Result,
	core::models::{
		certs::{CertDir, CertificateType, Email},
		proxy::{ProxyInputAddress, ProxyPort},
		routes::{Host, Route, Upstream},
		tasks::TaskInterval,
	},
};
use serde::Deserialize;
use std::{env, fs};

const CONFIG_PATH_ENV: &str = "CONFIG_PATH";
const CERT_DIR: &str = ".certs/";
const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;
const INPUT_ADDRESS: &str = "0.0.0.0";

// in seconds
const CERT_BACKGROUND_TASK_INTERVAL: u64 = 120;

#[derive(Debug, Clone)]
pub struct Config {
	pub email: Email,
	pub cert_dir: CertDir,
	pub routes: Vec<Route>,
	pub task_interval: TaskInterval,
	pub http_port: ProxyPort,
	pub https_port: ProxyPort,
	pub input_address: ProxyInputAddress,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigTomlFile {
	pub acme: Acme,
	pub routes: Vec<Route>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Acme {
	pub email: Email,
}

impl Config {
	pub fn init() -> Result<Self> {
		let config_path = load_env(CONFIG_PATH_ENV)?;
		let config_file = parse_toml_config(config_path)?;

		Ok(Config {
			email: config_file.acme.email.clone(),
			cert_dir: CertDir::from(CERT_DIR.to_string()),
			routes: config_file.routes.clone(),
			task_interval: TaskInterval::from(CERT_BACKGROUND_TASK_INTERVAL),
			http_port: ProxyPort::from(HTTP_PORT),
			https_port: ProxyPort::from(HTTPS_PORT),
			input_address: ProxyInputAddress::from(INPUT_ADDRESS.to_string()),
		})
	}
}

fn parse_toml_config(config_path: String) -> Result<ConfigTomlFile> {
	let content = fs::read_to_string(config_path)?;
	let config: ConfigTomlFile =
		toml::from_str(&content).map_err(|e| Error::Config(e.to_string()))?;

	Ok(config)
}

fn load_env(key: &str) -> Result<String> {
	match env::var(key) {
		Ok(val) => Ok(val),
		Err(_err) => Err(Error::Env(key.to_string())),
	}
}
