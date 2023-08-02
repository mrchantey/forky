use crate::server::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct SweetCli {
	pub package: Option<String>,
	pub server: Server,
}


impl SweetCli {
	pub fn set_package(&mut self, package: String) -> &mut Self {
		self.package = Some(package);
		self
	}
}

impl Default for SweetCli {
	fn default() -> Self {
		Self {
			package: None,
			server: Server {
				quiet: true,
				dir: "target/sweet".to_string(),
				..Server::default()
			},
		}
	}
}

impl Display for SweetCli {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(package) = &self.package {
			write!(f, "package: {package}")?;
		}
		Ok(())
	}
}
