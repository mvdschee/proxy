use crate::config::Config;
pub use error::{Error, Result};
use std::env;

mod config;
pub mod error;

fn main() -> Result<()> {
	let config = Config::init()?;

	dbg!(config);

	Err(Error::MainLoopClosed)
}
