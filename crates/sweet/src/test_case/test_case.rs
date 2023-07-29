use crate::TestCaseConfig;
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
		let mut val = String::from(
			&["\n● ", &self.filename(), " > ", self.name(), "\n\n"]
				.concat()
				.red()
				.bold()[..],
		);
		val.push_str(msg);
		val.push_str("\n\n");
		anyhow::anyhow!(val)
	}

	async fn run_func(&self) -> Result<()>;

	async fn run(&self) -> Result<()> { self.run_func().await }
}
