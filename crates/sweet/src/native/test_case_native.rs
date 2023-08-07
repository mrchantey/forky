use crate::*;
use anyhow::Result;
use forky_core::*;
use futures::Future;
use std::panic::UnwindSafe;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;

inventory::collect!(TestCaseNative);

pub type TestCaseNativeFn = fn() -> Pin<
	Box<dyn Send + Sync + UnwindSafe + Future<Output = Result<()>>>,
>;

#[derive(Debug, Clone)]
pub struct TestCaseNative {
	pub file: &'static str,
	pub name: &'static str,
	pub func: TestCaseNativeFn,
	pub config: TestCaseConfig,
}

impl TestCase for TestCaseNative {
	fn path(&self) -> PathBuf { Path::new(self.file).to_forward_slash() }
	fn name(&self) -> &str { self.name }
	fn config(&self) -> &TestCaseConfig { &self.config }
	async fn run_func(&self) -> Result<()> {
		let fut = (self.func)();
		std::panic::set_hook(Box::new(|_| {}));
		let result = unwrap_panic_async(fut).await;
		let _ = std::panic::take_hook();
		result
	}
	async fn run(&self) -> Result<()> {
		let result = self.run_func().await;
		match result {
			Ok(_) => Ok(()),
			Err(e) => Err(self.format_error(&e.to_string())),
		}
	}
}
