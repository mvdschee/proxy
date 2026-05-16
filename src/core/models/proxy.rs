use crate::core::models::{
	filesystem::SafePath,
	routes::{Host, Upstream},
};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ProxyRoute {
	pub host: Host,
	pub upstream: Upstream,
	pub tls: ProxyTls,
	pub cert_path: SafePath,
	pub key_path: SafePath,
}

// --- PROXY TLS ---
#[derive(Debug, Clone)]
pub struct ProxyTls(bool);

impl Deref for ProxyTls {
	type Target = bool;

	fn deref(&self) -> &bool {
		&self.0
	}
}

impl From<bool> for ProxyTls {
	fn from(s: bool) -> Self {
		ProxyTls(s)
	}
}
