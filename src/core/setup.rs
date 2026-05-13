use crate::{
	Result,
	config::Acme,
	core::models::{
		certs::{CertDir, Certificate, Email},
		routes::Route,
	},
	info,
};
use std::path::Path;
use tokio::spawn;

pub struct HandleFileSystem {
	cert_dir: CertDir,
}

impl HandleFileSystem {
	pub fn new(cert_dir: CertDir) -> Self {
		Self {
			cert_dir,
		}
	}

	pub async fn run(&self) -> Result<()> {
		info!("setup filesystem...");

		// foxguard: ignore[rs/no-path-traversal]
		if !Path::new(self.cert_dir.as_str()).exists() {
			info!("creating cert directory at {}", &self.cert_dir);
			std::fs::create_dir_all(self.cert_dir.as_str())?;
		} else {
			info!("cert directory already exists at {}", &self.cert_dir);
		}

		Ok(())
	}
}

pub struct HandleCertificates {
	certificates: Vec<Certificate>,
}

impl HandleCertificates {
	pub fn new(cert_dir: CertDir, email: Email, routes: Vec<Route>) -> Self {
		let certificates = routes
			.into_iter()
			.map(|route| Certificate {
				host: route.host.clone(),
				cert_dir: cert_dir.clone(),
				email: email.clone(),
				cert_type: route.cert_type.clone(),
			})
			.collect::<Vec<Certificate>>();

		dbg!(&certificates);

		Self {
			certificates: Vec::new(),
		}
	}

	pub async fn run(self) -> Result<()> {
		info!("starting certificates tasks...");

		spawn(async move {
			info!("running certificates tasks...");
		});

		Ok(())
	}
}
