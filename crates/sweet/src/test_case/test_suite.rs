use super::*;
use crate::SuiteLogger;
use crate::TestSuiteResult;
use anyhow::Error;
use rayon::prelude::*;


// pub trait TestSuiteImpl<Logger>
// where
// 	Self: Sized,
// 	Logger: SuiteLogger,
// {



// }


#[derive(Default, Debug, Clone)]
pub struct TestSuite<T>
where
	T: TestCase,
{
	pub file: String,
	pub tests: Vec<T>,
	pub contains_only: bool,
	pub config: TestCaseConfig,
}

impl<T> TestSuite<T>
where
	T: TestCase,
{
	pub fn new(file: String) -> Self {
		Self {
			file,
			tests: Vec::new(),
			contains_only: false,
			config: TestCaseConfig::Default,
		}
	}
	fn should_skip(&self, test: &T) -> bool {
		*test.config() == TestCaseConfig::Skip
			|| self.config == TestCaseConfig::Skip
			|| (self.contains_only && *test.config() != TestCaseConfig::Only)
	}

	pub fn run_strategy(to_run: Vec<&T>) -> Vec<Error> {
		to_run
			.iter()
			.map(|t| t.run())
			.filter_map(|result| result.err())
			.collect::<Vec<_>>()
	}
	pub fn run<Logger>(
		&self,
		config: &TestRunnerConfig,
		run_strategy: fn(Vec<&T>) -> Vec<Error>,
	) -> TestSuiteResult
	where
		Logger: SuiteLogger,
	{
		let running_indicator = !config.parallel;
		let logger = Logger::start(self.file.as_str(), running_indicator);

		let (to_run, skipped): (Vec<_>, Vec<_>) =
			self.tests.iter().partition(|t| !self.should_skip(t));

		let failed = run_strategy(to_run);

		let msg = failed
			.iter()
			.fold(String::new(), |val, err| val + err.to_string().as_str());
		Logger::log(&msg.as_str());
		// logger.get_log().push_str(&msg.as_str());

		let result = TestSuiteResult {
			tests: self.tests.len(),
			failed: failed.len(),
			skipped: skipped.len(),
		};
		logger.end(&self.file, running_indicator, &result);
		result
	}
}


impl<T> TestSuite<T>
where
	T: TestCase + Send + Sync,
{
	pub fn run_parallel_strategy(to_run: Vec<&T>) -> Vec<Error> {
		to_run
			.par_iter()
			.map(|t| t.run())
			.filter_map(|result| result.err())
			.collect::<Vec<_>>()
	}
}
