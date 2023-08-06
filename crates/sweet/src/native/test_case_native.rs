use crate::*;
use anyhow::Result;
use futures::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;

inventory::collect!(TestCaseNative);

pub type TestCaseNativeFn =
	fn() -> Pin<Box<dyn UnwindSafe + Future<Output = Result<()>>>>;

#[derive(Debug, Clone)]
pub struct TestCaseNative {
	pub file: &'static str,
	pub name: &'static str,
	pub func: TestCaseNativeFn,
	pub config: TestCaseConfig,
}

impl TestCase for TestCaseNative {
	fn file(&self) -> &str { self.file }
	fn name(&self) -> &str { self.name }
	fn config(&self) -> &TestCaseConfig { &self.config }
	async fn run_func(&self) -> Result<()> {
		let fut = (self.func)();
		std::panic::set_hook(Box::new(|_| {}));
		let result = unwrap_panic_async(fut).await;
		let _ = std::panic::take_hook();
		result
	}
}
