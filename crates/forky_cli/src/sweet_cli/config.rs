use std::fmt::Display;


#[derive(Debug, Clone)]
pub struct SweetCliConfig {
	pub package: Option<String>,
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
