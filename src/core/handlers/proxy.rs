use pingora::server::Server;

use crate::{
	Error, Result,
	core::models::{
		certs::{CertDir, CertificateType},
		proxy::ProxyRoute,
		routes::Route,
	},
};

pub fn run_proxy(cert_dir: CertDir, routes: Vec<ProxyRoute>) -> Result<()> {
	let mut server = Server::new(None).map_err(|e| Error::Proxy(e.to_string()))?;
	server.bootstrap();

	// tls proxies

	// redirect to tls proxies

	// plain proxies

	server.run_forever();

	Err(Error::Proxy("proxy stopped".to_string()))
}
