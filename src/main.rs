use crate::{
	config::Config,
	core::setup::{HandleCertificates, HandleFileSystem},
};
pub use error::{Error, Result};
use std::env;
use tokio::signal;

mod config;
mod core;
pub mod error;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
	let config = Config::init()?;

	// create cert directory if it doesn't exist yet
	let fs_handler = HandleFileSystem::new(config.cert_dir.clone());
	fs_handler.run().await?;

	// generate any missing certificates
	// and spin up background tasks to refresh certificates
	let cert_handler =
		HandleCertificates::new(config.cert_dir, config.email, config.routes, config.task_interval);

	cert_handler.run().await?;

	// start the proxy

	signal::ctrl_c().await.map_err(|_| Error::MainLoopClosed)?;

	Err(Error::MainLoopClosed)
}
