use crate::{
	Error, Result,
	config::Acme,
	core::{
		handlers::{
			certs::{background_certs_task, generate_certs},
			filesystem::{check_file_exists, safe_path},
			proxy::run_proxy,
		},
		models::{
			certs::{CertDir, Certificate, CertificateType, Email},
			proxy::{ProxyRoute, ProxyTls},
			routes::Route,
			tasks::TaskInterval,
		},
	},
	error, info, warn,
};
use std::path::Path;

pub struct HandleFileSystem {
	cert_dir: CertDir,
}

impl HandleFileSystem {
	pub fn new(cert_dir: CertDir) -> Self {
		Self {
			cert_dir,
		}
	}

	pub fn run(&self) -> Result<()> {
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

	pub fn run(self) -> Result<()> {
		generate_certs(self.certificates.clone())?;

		// spawn(async move {
		// 	info!("starting certificates tasks...");
		// 	background_certs_task(self.certificates, self.task_interval).await;
		// });

		Ok(())
	}
}

pub struct HandleProxy {
	cert_dir: CertDir,
	proxy_routes: Vec<ProxyRoute>,
}

impl HandleProxy {
	pub fn new(cert_dir: CertDir, routes: Vec<Route>) -> Result<Self> {
		let mut proxy_routes = Vec::new();

		for route in routes {
			let cert_filename = format!("{}.pem", route.host);
			let key_filename = format!("{}.key", route.host);

			let key_path = safe_path(&cert_dir, &key_filename)?;
			let cert_path = safe_path(&cert_dir, &cert_filename)?;

			let has_tls_files = check_file_exists(&key_path) && check_file_exists(&cert_path);

			// missing cert files but cert_type is not None, skip
			if !has_tls_files && route.cert_type != CertificateType::None {
				warn!("tls files not found for host `{}` but cert is expected", route.host);
			}

			proxy_routes.push(ProxyRoute {
				host: route.host.clone(),
				upstream: route.upstream.clone(),
				tls: ProxyTls::from(has_tls_files),
				cert_path,
				key_path,
			});
		}

		Ok(Self {
			cert_dir,
			proxy_routes,
		})
	}

	pub fn run(&self) -> Result<()> {
		info!("proxy running...");

		run_proxy(self.cert_dir.clone(), self.proxy_routes.clone())?;

		Err(Error::Proxy("proxy exited".to_string()))
	}
}
