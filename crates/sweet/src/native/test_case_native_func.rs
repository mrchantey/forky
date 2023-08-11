use anyhow::Result;
use futures::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;

pub type BoxedFuture = Pin<Box<dyn Future<Output = Result<()>>>>;
pub type BoxedFutureUnwindSafe =
	Pin<Box<dyn Send + Sync + Future<Output = Result<()>> + UnwindSafe>>;

pub type TestCaseNativeFuncSync = fn() -> Result<()>;
pub type TestCaseNativeFuncSeries = fn() -> BoxedFuture;
pub type TestCaseNativeFuncParallel = fn() -> BoxedFutureUnwindSafe;

#[derive(Debug, Clone)]
pub enum TestCaseNativeFunc {
	Sync(TestCaseNativeFuncSync),
	Series(TestCaseNativeFuncSeries),
	Parallel(TestCaseNativeFuncParallel),
}
