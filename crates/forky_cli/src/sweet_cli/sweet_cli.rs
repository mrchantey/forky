use super::*;
use crate::server::*;
use std::fmt::Display;


#[derive(Debug, Clone)]
pub struct SweetCli {
	pub example: String,
	pub matches: Vec<String>,
	pub package: Option<String>,
	pub release: bool,
	pub run_tests_mode: Option<RunTestsMode>,
	pub server: Server,
	pub static_dir: Option<String>,
	pub watch: bool,
}


impl SweetCli {
	pub fn should_run_once(&self) -> bool {
		self.run_tests_mode.is_some() && !self.watch
	}
	pub fn set_package(&mut self, package: String) -> &mut Self {
		self.package = Some(package);
		self
	}
}

impl Default for SweetCli {
	fn default() -> Self {
		Self {
			example: "sweet".to_string(),
			matches: Vec::new(),
			package: None,
			release: false,
			run_tests_mode: None,
			server: Server {
				quiet: true,
				proxy: true,
				dir: "target/sweet".to_string(),
				..Server::default()
			},
			static_dir: None,
			watch: true,
		}
	}
}

impl Display for SweetCli {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(package) = &self.package {
			//TODO
			write!(f, "package: {package}")?;
		}
		Ok(())
	}
}
