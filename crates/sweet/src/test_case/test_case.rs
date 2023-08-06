use super::*;
use anyhow::Result;
use colorize::*;
use std::panic::RefUnwindSafe;

pub trait TestCase
where
	Self: RefUnwindSafe,
{
	fn file(&self) -> &str;
	fn name(&self) -> &str;
	fn config(&self) -> &TestCaseConfig;
	fn filename(&self) -> String {
		let f = self.file().replace(".rs", "");
		let f = f.split('\\').last().unwrap();
		let f = f.trim();
		String::from(f)
	}
	fn format_error(&self, msg: &str) -> anyhow::Error {
		let location = format!("\n● {} > {}", self.filename(), self.name())
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
