use anyhow::Result;
use colorize::*;
use std::any::Any;
use std::panic::AssertUnwindSafe;
use std::panic::{self,};

inventory::collect!(TestCaseDesc);

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TestCaseConfig {
	#[default]
	Default,
	Skip,
	Only,
}
impl TestCaseConfig {
	pub fn to_i32(&self) -> i32 {
		match self {
			Self::Default => 0,
			Self::Skip => 1,
			Self::Only => 2,
		}
	}
	pub fn from_i32(val: i32) -> Self {
		match val {
			0 => Self::Default,
			1 => Self::Skip,
			2 => Self::Only,
			_ => Self::Default,
		}
	}
}

#[derive(Debug, Clone)]
pub struct TestCaseDesc {
	pub file: &'static str,
	pub name: &'static str,
	pub func: fn() -> Result<()>,
	pub config: TestCaseConfig,
}

impl TestCaseDesc {
	pub fn filename(&self) -> String {
		let f = self.file.replace(".rs", "");
		let f = f.split('\\').last().unwrap();
		let f = f.trim();
		String::from(f)
	}

	pub fn format_error(&self, msg: &str) -> anyhow::Error {
		let mut val = String::from(
			&["\n● ", &self.filename(), " > ", self.name, "\n\n"]
				.concat()
				.red()
				.bold()[..],
		);
		val.push_str(msg);
		val.push_str("\n\n");
		anyhow::anyhow!(val)
	}

	pub fn run(&self) -> Result<()> {
		let panic_res = panic::catch_unwind(AssertUnwindSafe(|| (self.func)()));
		let _ = panic::take_hook();
		match panic_res {
			Ok(matcher_res) => match matcher_res {
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
