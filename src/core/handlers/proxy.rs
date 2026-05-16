use crate::{
	Error, Result,
	core::models::proxy::{ProxyConfig, ProxyRoute},
};
use async_trait::async_trait;
use http::{Response, header};
use pingora::{
	apps::http_app::ServeHttp,
	http::StatusCode,
	prelude::{HttpPeer, Result as PingoraResult},
	protocols::http::ServerSession,
	proxy::{ProxyHttp, Session, http_proxy_service},
	server::{Server, configuration::ServerConf},
	services::{Service, listening::Service as ListeningService},
};
use std::sync::Arc;

static BODY: &[u8] = "<html><body>301 Moved Permanently</body></html>".as_bytes();

pub fn run_proxy(proxy_config: ProxyConfig, routes: Vec<ProxyRoute>) -> Result<()> {
	let mut server = Server::new(None).map_err(|e| Error::Proxy(e.to_string()))?;

	server.bootstrap();

	let server_conf = server.configuration.clone();
	let http_addr = format!("{}:{}", proxy_config.input_address, *proxy_config.http_port);
	let https_addr = format!("{}:{}", proxy_config.input_address, *proxy_config.https_port);

	// tls proxies
	let tls_services =
		tls_routes_services(server_conf.clone(), https_addr.clone(), routes.clone())?;

	for service in tls_services {
		server.add_service(service);
	}

	// redirect to tls proxies
	let redirect_service = http_redirect_service(http_addr.clone());

	server.add_service(redirect_service);
	// plain proxies
	let plain_services = plain_routes_services(server_conf, http_addr.clone(), routes.clone())?;

	for service in plain_services {
		server.add_service(service);
	}

	server.run_forever();

	Err(Error::Proxy("proxy stopped".to_string()))
}

pub fn tls_routes_services(
	server_conf: Arc<ServerConf>,
	listen_addr: String,
	routes: Vec<ProxyRoute>,
) -> Result<Vec<impl Service>> {
	let mut services = Vec::new();

	let tls_routes = routes.into_iter().filter(|route| *route.tls == true).collect::<Vec<_>>();

	for route in tls_routes {
		let proxy_app = ProxyToUpstream::new(route.clone());

		let mut service = http_proxy_service(&server_conf, proxy_app);
		service
			.add_tls(&listen_addr, &route.cert_path, &route.key_path)
			.map_err(|e| Error::Proxy(e.to_string()));

		services.push(service);
	}

	Ok(services)
}

pub fn plain_routes_services(
	server_conf: Arc<ServerConf>,
	listen_addr: String,
	routes: Vec<ProxyRoute>,
) -> Result<Vec<impl Service>> {
	let mut services = Vec::new();

	let plain_routes = routes.into_iter().filter(|route| *route.tls == false).collect::<Vec<_>>();

	for route in plain_routes {
		let proxy_app = ProxyToUpstream::new(route.clone());

		let mut service = http_proxy_service(&server_conf, proxy_app);
		service.add_tcp(&listen_addr);

		services.push(service);
	}

	Ok(services)
}

fn http_redirect_service(listen_addr: String) -> impl Service {
	let mut service = ListeningService::new("HTTPS Redirect".to_string(), RedirectToHttps {});
	service.add_tcp(&listen_addr);

	service
}

pub struct ProxyToUpstream {
	route: ProxyRoute,
}

impl ProxyToUpstream {
	pub fn new(route: ProxyRoute) -> Self {
		Self {
			route,
		}
	}
}

#[async_trait]
impl ProxyHttp for ProxyToUpstream {
	type CTX = ();
	fn new_ctx(&self) {}

	async fn upstream_peer(
		&self,
		session: &mut Session,
		_ctx: &mut (),
	) -> PingoraResult<Box<HttpPeer>> {
		// foxguard: ignore[rs/no-unwrap-in-lib]
		let host_header = session.get_header(header::HOST).unwrap().to_str().unwrap();

		let proxy_to =
			HttpPeer::new(self.route.upstream.as_str(), false, self.route.host.to_string());
		let peer = Box::new(proxy_to);
		Ok(peer)
	}
}

pub struct RedirectToHttps;

#[async_trait]
impl ServeHttp for RedirectToHttps {
	async fn response(&self, http_stream: &mut ServerSession) -> Response<Vec<u8>> {
		// foxguard: ignore[rs/no-unwrap-in-lib]
		let host_header = http_stream.get_header(header::HOST).unwrap().to_str().unwrap();

		Response::builder()
			.status(StatusCode::MOVED_PERMANENTLY)
			.header(header::CONTENT_TYPE, "text/html")
			.header(header::CONTENT_LENGTH, BODY.len())
			.header(header::LOCATION, format!("https://{host_header}"))
			.body(BODY.to_owned())
			// foxguard: ignore[rs/no-unwrap-in-lib]
			.unwrap()
	}
}
