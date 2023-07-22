use super::*;
use crate::TestLogger;
use crate::TestSuiteResult;
use rayon::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct TestSuite<T>
where
	T: TestCase,
{
	pub file: &'static str,
	pub tests: Vec<T>,
	pub contains_only: bool,
	pub config: TestCaseConfig,
}

impl<T> TestSuite<T>
where
	T: TestCase,
{
	pub fn new(file: &'static str) -> Self {
		Self {
			file,
			tests: Vec::new(),
			contains_only: false,
			config: TestCaseConfig::Default,
		}
	}
}

impl<T> TestSuite<T>
where
	T: TestCase + Send + Sync,
{
	fn should_skip(&self, test: &T) -> bool {
		*test.config() == TestCaseConfig::Skip
			|| self.config == TestCaseConfig::Skip
			|| (self.contains_only && *test.config() != TestCaseConfig::Only)
	}

	pub fn run(&self, config: &TestRunnerConfig) -> TestSuiteResult {
		let mut logger = TestLogger::start(self.file, !config.parallel);

		let (to_run, skipped): (Vec<_>, Vec<_>) =
			self.tests.iter().partition(|t| !self.should_skip(t));

		let failed = if config.parallel {
			to_run
				.par_iter()
				.map(|t| t.run())
				.filter_map(|result| result.err())
				.collect::<Vec<_>>()
		} else {
			to_run
				.iter()
				.map(|t| t.run())
				.filter_map(|result| result.err())
				.collect::<Vec<_>>()
		};


		let msg = failed
			.iter()
			.fold(String::new(), |val, err| val + err.to_string().as_str());
		logger.log.push_str(&msg.to_string());

		let result = TestSuiteResult {
			tests: self.tests.len(),
			failed: failed.len(),
			skipped: skipped.len(),
		};
		logger.end(&result);
		result
	}
}
