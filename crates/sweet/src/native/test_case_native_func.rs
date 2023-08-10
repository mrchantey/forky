use crate::*;
use anyhow::Result;
use forky_core::*;
use futures::Future;
use std::any::Any;
use std::panic::UnwindSafe;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;

pub type BoxedFuture = Pin<Box<dyn Send + Sync + Future<Output = Result<()>>>>;
pub type BoxedFutureUnwindSafe =
	Pin<Box<dyn Send + Sync + Future<Output = Result<()>> + UnwindSafe>>;

pub type TestCaseNativeFuncSync = fn() -> Result<()>;
pub type TestCaseNativeFuncSeries = fn() -> BoxedFuture;
pub type TestCaseNativeFuncParallel = fn() -> BoxedFutureUnwindSafe;

pub enum TestCaseNativeFunc {
	Sync(TestCaseNativeFuncSync),
	Series(TestCaseNativeFuncSeries),
	Parallel(TestCaseNativeFuncParallel),
}

impl TestCaseNativeFunc {
	pub fn split_funcs<'a>(
		funcs: impl IntoIterator<Item = &'a Self>,
	) -> (
		Vec<&'a TestCaseNativeFuncSync>,
		Vec<&'a TestCaseNativeFuncSeries>,
		Vec<&'a TestCaseNativeFuncParallel>,
	) {
		funcs.into_iter().fold(
			(Vec::new(), Vec::new(), Vec::new()),
			|(mut syncs, mut series, mut parallels), f| {
				match f {
					TestCaseNativeFunc::Sync(f) => syncs.push(f),
					TestCaseNativeFunc::Series(f) => series.push(f),
					TestCaseNativeFunc::Parallel(f) => parallels.push(f),
				}
				(syncs, series, parallels)
			},
		)
	}
}
