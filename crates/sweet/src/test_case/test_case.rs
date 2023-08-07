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
	fn format_error(&self, msg: &str) -> anyhow::Error {
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
	}

	async fn run_func(&self) -> Result<()>;

	async fn run(&self) -> Result<()> {
		let result = self.run_func().await;
		match result {
			Ok(_) => Ok(()),
			Err(e) => Err(self.format_error(&e.to_string())),
		}
	}
}
