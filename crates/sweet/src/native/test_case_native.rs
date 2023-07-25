use crate::*;
use anyhow::Result;

inventory::collect!(TestCaseNative);


#[derive(Debug, Clone)]
pub struct TestCaseNative {
	pub file: &'static str,
	pub name: &'static str,
	pub func: fn() -> Result<()>,
	pub config: TestCaseConfig,
}



impl TestCase for TestCaseNative {
	fn file(&self) -> &str { self.file }
	fn name(&self) -> &str { self.name }
	fn config(&self) -> &TestCaseConfig { &self.config }
	//TODO async
	async fn run_func(&self) -> anyhow::Result<()> { (self.func)() }
}
