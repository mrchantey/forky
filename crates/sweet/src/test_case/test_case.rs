use super::*;
use anyhow::Result;
use colorize::*;
use std::panic::RefUnwindSafe;
use std::path::PathBuf;

pub trait TestCase
where
	Self: RefUnwindSafe,
{
	fn path(&self) -> PathBuf;
	fn name(&self) -> &str;
	fn config(&self) -> &TestCaseConfig;
	fn format_error(&self, result: anyhow::Result<()>) -> anyhow::Result<()> {
		result.map_err(|error| {
			let msg = error.to_string();
			let location = format!(
				"\n● {} > {}",
				self.path()
					.file_stem()
					.unwrap_or_default()
					.to_string_lossy(),
				self.name()
			)
			.red()
			.bold();
			let val = format!("{location}\n\n{msg}\n");
			anyhow::anyhow!(val)
		})
	}

	async fn run_func(&self) -> Result<()>;

	async fn run(&self) -> Result<()> {
		let result = self.run_func().await;
		self.format_error(result)
	}
}
