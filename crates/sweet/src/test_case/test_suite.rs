use super::*;
use crate::SuiteLogger;
use crate::TestSuiteResult;
use anyhow::Error;

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

	pub fn run<Logger, Runner>(
		&self,
		config: &TestRunnerConfig,
	) -> TestSuiteResult
	where
		Logger: SuiteLogger,
		Runner: TestSuiteRunner<T>,
	{
		let running_indicator = !config.parallel;
		let logger = Logger::start(self.file.as_str(), running_indicator);

		let (to_run, skipped): (Vec<_>, Vec<_>) =
			self.tests.iter().partition(|t| !self.should_skip(t));

		let failed = Runner::run_cases(to_run, config);

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

pub trait TestSuiteRunner<Case>
where
	Self: Sized,
	Case: TestCase,
{
	fn run_cases(to_run: Vec<&Case>, _: &TestRunnerConfig) -> Vec<Error>;
}
pub struct TestSuiteRunnerSeries<Case> {
	_case: std::marker::PhantomData<Case>,
}

impl<Case> TestSuiteRunner<Case> for TestSuiteRunnerSeries<Case>
where
	Case: TestCase,
{
	fn run_cases(to_run: Vec<&Case>, _: &TestRunnerConfig) -> Vec<Error> {
		to_run
			.iter()
			.map(|t| t.run())
			.filter_map(|result| result.err())
			.collect::<Vec<_>>()
	}
}


pub struct TestSuiteRunnerParallel<Case> {
	_case: std::marker::PhantomData<Case>,
}

impl<Case> TestSuiteRunner<Case> for TestSuiteRunnerParallel<Case>
where
	Case: TestCase + Send + Sync,
{
	fn run_cases(to_run: Vec<&Case>, _: &TestRunnerConfig) -> Vec<Error> {
		use rayon::prelude::*;
		to_run
			.par_iter()
			.map(move |t| t.run())
			.filter_map(|result| result.err())
			.collect::<Vec<_>>()
	}
}
