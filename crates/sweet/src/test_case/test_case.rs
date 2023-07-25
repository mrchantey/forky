use crate::TestCaseConfig;
use anyhow::Result;
use colorize::*;
use std::any::Any;
use std::panic::AssertUnwindSafe;
use std::panic::{self,};

pub trait TestCase {
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

	async fn run(&self) -> Result<()> {
		let panic_res =
			panic::catch_unwind(AssertUnwindSafe(async || self.run_func().await));

		#[cfg(not(target_arch = "wasm32"))]
		let _ = panic::take_hook();
		match panic_res {
			Ok(matcher_res) => match matcher_res.await {
				Ok(()) => Ok(()),
				Err(err) => Err(self.format_error(&err.to_string().as_str())),
			},
			Err(e) => Err(self.format_error(&panic_info(e).as_str())),
		}
	}
}

fn panic_info(e: Box<dyn Any + Send>) -> String {
	match e.downcast::<String>() {
		Ok(v) => *v,
		Err(e) => match e.downcast::<&str>() {
			Ok(v) => v.to_string(),
			_ => "Unknown Source of Error".to_owned(),
		},
	}
}
