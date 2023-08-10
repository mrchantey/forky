use crate::*;
use anyhow::Result;
use forky_core::*;
use std::any::Any;
use std::path::Path;
use std::path::PathBuf;

inventory::collect!(TestCaseNative);

// pub type TestCaseNativeFn =
// 	fn() -> Pin<Box<dyn Send + Sync + Future<Output = Result<()>>>>;
// pub type TestCaseNativeFnUnwindSafe = fn() -> Pin<
// 	Box<dyn Send + Sync + Future<Output = Result<()>> + UnwindSafe>,
// >;

#[derive(Debug, Clone)]
pub struct TestCaseNative {
	pub file: &'static str,
	pub name: &'static str,
	pub func: TestCaseNativeFuncParallel,
	pub config: TestCaseConfig,
}

async fn try_catch_panic(fut: BoxedFuture) -> Result<()> {
	let fut: Box<dyn Any + Send + Sync> = Box::new(fut);

	match fut.downcast::<Box<BoxedFutureUnwindSafe>>() {
		Ok(fut) => {
			// println!("its unwind safe!");
			// std::panic::set_hook(Box::new(|_| {}));
			let result = unwrap_panic_async(**fut).await;
			// let _ = std::panic::take_hook();
			result
		}
		Err(fut) => {
			// println!("its not safe!");
			let fut = fut.downcast::<BoxedFuture>().unwrap();
			fut.await
		}
	}
}

impl TestCase for TestCaseNative {
	/// Forward slashed path
	fn path(&self) -> PathBuf { Path::new(self.file).to_forward_slash() }
	fn name(&self) -> &str { self.name }
	fn config(&self) -> &TestCaseConfig { &self.config }
	async fn run_func(&self) -> Result<()> {
		//TODO panic!("native test cases are run by the native suite");
		let fut = (self.func)();
		try_catch_panic(fut).await
	}
}
