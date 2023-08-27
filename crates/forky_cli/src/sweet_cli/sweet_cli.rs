use super::*;
use crate::server::*;
use std::fmt::Display;


#[derive(Debug, Clone)]
pub struct SweetCli {
	pub package: Option<String>,
	pub server: Server,
	pub release: bool,
	pub watch: bool,
	pub static_dir: Option<String>,
	pub run_tests: Option<RunTestsMode>,
}


impl SweetCli {
	pub fn should_run_once(&self) -> bool {
		self.run_tests.is_some() && !self.watch
	}
	pub fn set_package(&mut self, package: String) -> &mut Self {
		self.package = Some(package);
		self
	}
}

impl Default for SweetCli {
	fn default() -> Self {
		Self {
			package: None,
			static_dir: None,
			run_tests: None,
			watch: true,
			server: Server {
				quiet: true,
				proxy: true,
				dir: "target/sweet".to_string(),
				..Server::default()
			},
			release: false,
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
