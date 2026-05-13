use std::{fmt, ops::Deref};

// --- SAFEPATH ---
#[derive(Debug, Clone)]
pub struct SafePath(String);

impl Deref for SafePath {
	type Target = String;

	fn deref(&self) -> &String {
		&self.0
	}
}

impl fmt::Display for SafePath {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<String> for SafePath {
	fn from(s: String) -> Self {
		SafePath(s)
	}
}
