use crate::*;
use anyhow::Result;
use futures::Future;
use std::pin::Pin;

inventory::collect!(TestCaseNative);

#[derive(Debug, Clone)]
pub struct TestCaseNative {
	pub file: &'static str,
	pub name: &'static str,
	pub func: fn() -> Pin<Box<dyn Future<Output = Result<()>>>>,
	pub config: TestCaseConfig,
}

impl TestCase for TestCaseNative {
	fn file(&self) -> &str { self.file }
	fn name(&self) -> &str { self.name }
	fn config(&self) -> &TestCaseConfig { &self.config }
	async fn run_func(&self) -> Result<()> {
		match (self.func)().await {
			Ok(()) => Ok(()),
			Err(err) => Err(self.format_error(&err.to_string())),
		}
	}
}
