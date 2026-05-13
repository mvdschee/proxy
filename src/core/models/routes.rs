use crate::core::models::certs::CertificateType;
use serde::Deserialize;
use std::{fmt, ops::Deref};

#[derive(Debug, Clone, Deserialize)]
pub struct Route {
	pub host: Host,
	pub upstream: Upstream,
	#[serde(default)]
	pub cert_type: CertificateType,
}

// --- HOST ---
#[derive(Debug, Clone, Deserialize)]
pub struct Host(String);

impl Deref for Host {
	type Target = String;

	fn deref(&self) -> &String {
		&self.0
	}
}

impl fmt::Display for Host {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<String> for Host {
	fn from(s: String) -> Self {
		Host(s)
	}
}

// --- UPSTREAM ---
#[derive(Debug, Clone, Deserialize)]
pub struct Upstream(String);

impl Deref for Upstream {
	type Target = String;

	fn deref(&self) -> &String {
		&self.0
	}
}

impl fmt::Display for Upstream {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<String> for Upstream {
	fn from(s: String) -> Self {
		Upstream(s)
	}
}
