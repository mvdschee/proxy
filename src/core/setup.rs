use crate::{
	Result,
	config::Acme,
	core::{
		handlers::certs::{background_certs_task, initial_certs},
		models::{
			certs::{CertDir, Certificate, Email},
			routes::Route,
			tasks::TaskInterval,
		},
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
		// no user input is used here, so we can safely use Path::new directly
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
	task_interval: TaskInterval,
}

impl HandleCertificates {
	pub fn new(
		cert_dir: CertDir,
		email: Email,
		routes: Vec<Route>,
		task_interval: TaskInterval,
	) -> Self {
		let certificates = routes
			.into_iter()
			.map(|route| Certificate {
				host: route.host.clone(),
				cert_dir: cert_dir.clone(),
				email: email.clone(),
				cert_type: route.cert_type.clone(),
			})
			.collect::<Vec<Certificate>>();

		Self {
			certificates,
			task_interval,
		}
	}

	pub async fn run(self) -> Result<()> {
		initial_certs(self.certificates.clone()).await?;

		spawn(async move {
			info!("starting certificates tasks...");
			background_certs_task(self.certificates, self.task_interval).await;
		});

		Ok(())
	}
}
