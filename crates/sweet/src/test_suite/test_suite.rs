use crate::*;
use anyhow::Error;
use futures::Future;
use std::path::PathBuf;


pub async fn run_cases_series(to_run: Vec<&impl TestCase>) -> Vec<Error> {
	let mut results = Vec::with_capacity(to_run.len());
	for case in to_run {
		if let Err(result) = case.run().await {
			results.push(result);
		}
	}
	results
}

pub async fn run_cases_series_with_before<Case, Fut>(
	to_run: Vec<&Case>,
	mut before: impl FnMut(&Case) -> Fut,
) -> Vec<Error>
where
	Case: TestCase,
	Fut: Future<Output = ()>,
{
	let mut results = Vec::with_capacity(to_run.len());
	for case in to_run {
		before(case).await;
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
	fn new(file: PathBuf) -> Self;
	fn file(&self) -> &PathBuf;
	fn config(&self) -> &TestSuiteConfig;
	fn tests(&self) -> &Vec<Case>;
	fn tests_mut(&mut self) -> &mut Vec<Case>;
	fn push_test(&mut self, test: Case) { self.tests_mut().push(test) }

	fn should_skip(&self, test: &Case, contains_only: bool) -> bool {
		test.config().skip
			|| self.config().cases.skip
			|| (contains_only && !test.config().only)
	}

	async fn run_cases(
		&self,
		to_run: Vec<&Case>,
		_config: &TestRunnerConfig,
	) -> Vec<Error> {
		run_cases_series(to_run).await
	}

	async fn run<Logger>(&self, config: &TestRunnerConfig) -> SuiteResult
	where
		Logger: SuiteLogger,
	{
		let tests = self.tests();
		let file = self.file();

		let contains_only = tests.iter().any(|t| t.config().only);

		let (to_run, skipped): (Vec<_>, Vec<_>) = tests
			.iter()
			.partition(|t| !self.should_skip(t, contains_only));

		let mut result =
			SuiteResult::new(file.clone(), tests.len(), skipped.len());

		let logger = if config.silent {
			None
		} else {
			Some(Logger::on_start(result.in_progress_str()))
		};
		result.failed = self
			.run_cases(to_run, config)
			.await
			.iter()
			.map(|e| e.to_string())
			.collect();

		if let Some(logger) = logger {
			logger.on_end(result.end_str());
		}
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
