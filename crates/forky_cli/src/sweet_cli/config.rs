use std::fmt::Display;


#[derive(Debug, Clone)]
pub struct SweetCliConfig {
	pub package: Option<String>,
}


impl SweetCliConfig {
	pub fn set_package(&mut self, package: String) -> &mut Self {
		self.package = Some(package);
		self
	}
}

impl Default for SweetCliConfig {
	fn default() -> Self { Self { package: None } }
}

impl Display for SweetCliConfig {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(package) = &self.package {
			write!(f, "package: {package}")?;
		}
		Ok(())
	}
}
