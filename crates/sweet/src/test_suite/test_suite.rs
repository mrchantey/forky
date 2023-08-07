use crate::*;
use anyhow::Error;


pub async fn run_cases_series(
	to_run: Vec<&impl TestCase>,
	_: &TestRunnerConfig,
) -> Vec<Error> {
	let mut results = Vec::with_capacity(to_run.len());
	for case in to_run {
		if let Err(result) = case.run().await {
			results.push(result);
		}
	}
	results
}

pub trait TestSuiteTrait<Case>
where
	Case: TestCase,
{
	fn new(file: String) -> Self;
	fn file(&self) -> &str;
	fn config(&self) -> &TestSuiteConfig;
	fn tests(&self) -> &Vec<Case>;
	fn tests_mut(&mut self) -> &mut Vec<Case>;
	fn push_test(&mut self, test: Case) { self.tests_mut().push(test) }

	fn should_skip(&self, test: &Case, contains_only: bool) -> bool {
		*test.config() == TestCaseConfig::Skip
			|| self.config().skip
			|| (contains_only && *test.config() != TestCaseConfig::Only)
	}

	async fn run_cases(
		to_run: Vec<&Case>,
		config: &TestRunnerConfig,
	) -> Vec<Error> {
		run_cases_series(to_run, config).await
	}

	async fn run<Logger>(&self, config: &TestRunnerConfig) -> TestSuiteResult
	where
		Logger: SuiteLogger,
	{
		let tests = self.tests();
		let file = self.file();
		let logger = Logger::start(file);

		let contains_only =
			tests.iter().any(|t| *t.config() == TestCaseConfig::Only);

		let (to_run, skipped): (Vec<_>, Vec<_>) = tests
			.iter()
			.partition(|t| !self.should_skip(t, contains_only));

		let failed = Self::run_cases(to_run, config).await;

		let msg = failed
			.iter()
			.fold(String::new(), |val, err| val + err.to_string().as_str());

		let result = TestSuiteResult {
			tests: tests.len(),
			failed: failed.len(),
			skipped: skipped.len(),
		};
		logger.end(file, &result, msg);
		result
	}
}

// #[derive(Default, Debug, Clone)]
// pub struct TestSuite<Case>
// where
// 	Case: TestCase,
// {
// 	pub file: String,
// 	pub tests: Vec<Case>,
// 	pub config: TestSuiteConfig,
// }

// impl<Case> TestSuiteTrait<Case> for TestSuite<Case>
// where
// 	Case: TestCase,
// {
// 	fn new(file: String) -> Self {
// 		Self {
// 			file,
// 			tests: Vec::new(),
// 			config: Default::default(),
// 		}
// 	}
// 	fn file(&self) -> &str { self.file.as_str() }
// 	fn config(&self) -> &TestSuiteConfig { &self.config }
// 	fn tests(&self) -> &Vec<Case> { &self.tests }
// 	fn tests_mut(&mut self) -> &mut Vec<Case> { &mut self.tests }
// }
