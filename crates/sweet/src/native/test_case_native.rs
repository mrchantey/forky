use crate::test_case::*;
use crate::*;
use anyhow::Result;
use forky_core::*;
use std::path::Path;
use std::path::PathBuf;

inventory::collect!(TestCaseNative);

#[derive(Debug, Clone)]
pub struct TestCaseNative {
	pub file: &'static str,
	pub name: &'static str,
	pub func: TestCaseNativeFunc,
	pub config: TestCaseConfig,
}

impl TestCase for TestCaseNative {
	/// Forward slashed path
	fn path(&self) -> PathBuf { Path::new(self.file).to_forward_slash() }
	fn name(&self) -> &str { self.name }
	fn config(&self) -> &TestCaseConfig { &self.config }
	async fn run_func(&self) -> Result<()> {
		panic!("native test cases are run by the native suite");
		// let fut = (self.func)();
		// try_catch_panic(fut).await
	}
}
impl TestCaseNative {
	pub fn split_funcs<'a>(
		funcs: impl IntoIterator<Item = &'a Self>,
	) -> TestCaseNativeSplit<'a> {
		funcs.into_iter().fold(
			TestCaseNativeSplit::<'a>::default(),
			|mut split, t| {
				match &t.func {
					TestCaseNativeFunc::Sync(f) => split.syncs.push((t, f)),
					TestCaseNativeFunc::Series(f) => split.series.push((t, f)),
					TestCaseNativeFunc::Parallel(f) => {
						split.parallels.push((t, f))
					}
				}
				split
			},
		)
	}
}

#[derive(Debug, Default)]
pub struct TestCaseNativeSplit<'a> {
	pub syncs: Vec<(&'a TestCaseNative, &'a TestCaseNativeFuncSync)>,
	pub series: Vec<(&'a TestCaseNative, &'a TestCaseNativeFuncSeries)>,
	pub parallels: Vec<(&'a TestCaseNative, &'a TestCaseNativeFuncParallel)>,
}
