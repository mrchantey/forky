use crate::*;
use anyhow::Error;

#[derive(Default, Debug, Clone)]
pub struct TestSuite<Case>
where
	Case: TestCase,
{
	pub file: String,
	pub tests: Vec<Case>,
	pub config: TestSuiteConfig,
}

// pub trait TestSuite<Case>{



// }


impl<Case> TestSuite<Case>
where
	Case: TestCase,
{
	pub fn new(file: String) -> Self {
		Self {
			file,
			tests: Vec::new(),
			config: Default::default(),
		}
	}
	fn should_skip(&self, test: &Case, contains_only: bool) -> bool {
		*test.config() == TestCaseConfig::Skip
			|| self.config.skip
			|| (contains_only && *test.config() != TestCaseConfig::Only)
	}

	pub async fn run<Logger, Runner>(
		&self,
		config: &TestRunnerConfig,
	) -> TestSuiteResult
	where
		Logger: SuiteLogger,
		Runner: TestSuiteRunner<Case>,
	{
		let running_indicator = !config.parallel;
		let logger = Logger::start(self.file.as_str(), running_indicator);

		let contains_only = self
			.tests
			.iter()
			.any(|t| *t.config() == TestCaseConfig::Only);


		let (to_run, skipped): (Vec<_>, Vec<_>) = self
			.tests
			.iter()
			.partition(|t| !self.should_skip(t, contains_only));


		let failed = Runner::run_cases(to_run, config).await;

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
	async fn run_cases(to_run: Vec<&Case>, _: &TestRunnerConfig) -> Vec<Error>;
}
pub struct TestSuiteRunnerSeries<Case> {
	_case: std::marker::PhantomData<Case>,
}

impl<Case> TestSuiteRunner<Case> for TestSuiteRunnerSeries<Case>
where
	Case: TestCase,
{
	async fn run_cases(to_run: Vec<&Case>, _: &TestRunnerConfig) -> Vec<Error> {
		let mut results = Vec::with_capacity(to_run.len());
		for case in to_run {
			if let Err(result) = case.run().await {
				results.push(result);
			}
		}
		results
	}
}
