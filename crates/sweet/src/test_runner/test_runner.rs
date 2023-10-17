use super::*;
use crate::test_suite::*;
use crate::*;

pub struct TestRunner;

impl TestRunner {
	pub async fn run_group_series<Logger, Case>(
		to_run: Vec<&impl TestSuiteTrait<Case>>,
		config: &TestRunnerConfig,
	) -> TestRunnerResult
	where
		Case: TestCase,
		Logger: SuiteLogger,
	{
		let mut results = Vec::with_capacity(to_run.len());
		for suite in to_run {
			let result = suite.run::<Logger>(config).await;
			results.push(result);
		}
		results.into()
	}
}
