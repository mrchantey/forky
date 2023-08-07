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

pub trait TestSuiteTrait<Case>
where
	Case: TestCase,
{
	fn file(&self) -> &str;
	fn config(&self) -> &TestSuiteConfig;
	fn tests(&self) -> &Vec<Case>;

	fn should_skip(&self, test: &Case, contains_only: bool) -> bool {
		*test.config() == TestCaseConfig::Skip
			|| self.config().skip
			|| (contains_only && *test.config() != TestCaseConfig::Only)
	}

	async fn run<Logger, Runner>(
		&self,
		config: &TestRunnerConfig,
	) -> TestSuiteResult
	where
		Logger: SuiteLogger,
		Runner: TestSuiteRunner<Case>,
	{
		let tests = self.tests();
		let file = self.file();
		let running_indicator = !config.parallel;
		let logger = Logger::start(file, running_indicator);


		let contains_only =
			tests.iter().any(|t| *t.config() == TestCaseConfig::Only);


		let (to_run, skipped): (Vec<_>, Vec<_>) = tests
			.iter()
			.partition(|t| !self.should_skip(t, contains_only));


		let failed = Runner::run_cases(to_run, config).await;

		let msg = failed
			.iter()
			.fold(String::new(), |val, err| val + err.to_string().as_str());
		Logger::log(&msg.as_str());
		// logger.get_log().push_str(&msg.as_str());

		let result = TestSuiteResult {
			tests: tests.len(),
			failed: failed.len(),
			skipped: skipped.len(),
		};
		logger.end(file, running_indicator, &result);
		result
	}
}

impl<Case> TestSuiteTrait<Case> for TestSuite<Case>
where
	Case: TestCase,
{
	fn file(&self) -> &str { self.file.as_str() }
	fn config(&self) -> &TestSuiteConfig { &self.config }
	fn tests(&self) -> &Vec<Case> { &self.tests }
}


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
