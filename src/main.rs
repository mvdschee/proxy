use crate::{
	config::Config,
	core::setup::{HandleCertificates, HandleFileSystem, HandleProxy},
};
pub use error::{Error, Result};
use std::env;

mod config;
mod core;
pub mod error;
mod utils;

// entry needs to be synchronous as pingora has there own
// async runtime with tokio so we can't use tokio spawn
fn main() -> Result<()> {
	let config = Config::init()?;

	// create cert directory if it doesn't exist yet
	let fs_handler = HandleFileSystem::new(config.cert_dir.clone());
	fs_handler.run()?;

	// generate any missing certificates
	// and spin up background tasks to refresh certificates
	let cert_handler = HandleCertificates::new(
		config.cert_dir.clone(),
		config.email,
		config.routes.clone(),
		config.task_interval,
	);
	cert_handler.run()?;

	// start the proxy
	let proxy_handler = HandleProxy::new(config.cert_dir, config.routes)?;
	proxy_handler.run()?;

	Err(Error::MainLoopClosed)
}
