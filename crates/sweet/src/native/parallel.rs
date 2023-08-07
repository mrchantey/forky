use crate::*;
use futures::future::join_all;

//TODO generic, something like
// where
// Case: TestCase + Clone + Send + Sync,
// Suite: TestSuiteTrait<Case> + Clone + Send + Sync + Sized,
pub async fn run_cases_parallel(
	to_run: Vec<&TestCaseNative>,
	_: &TestRunnerConfig,
) -> Vec<anyhow::Error> {
	let futs = to_run
		.iter()
		.map(move |t| {
			let t = *t;
			let t = t.clone();
			tokio::spawn(async move { t.run().await })
		})
		.collect::<Vec<_>>();
	let results = join_all(futs).await;

	let mut collected = Vec::with_capacity(results.len());
	for join_result in results {
		match join_result {
			Ok(test_result) => match test_result {
				Ok(_) => {}
				Err(e) => collected.push(e),
			},
			Err(e) => {
				panic!("Join Error, this shouldnt happen:\n{:?}", e)
			}
		}
	}
	collected
}

impl TestRunner {
	pub async fn run_group_parallel<Logger, Case>(
		to_run: Vec<&impl TestSuiteTrait<Case>>,
		config: &TestRunnerConfig,
	) -> ResultSummary
	where
		Logger: SuiteLogger,
		Case: TestCase + Clone + Send + Sync,
	{
		let mut results = Vec::with_capacity(to_run.len());
		for suite in to_run {
			let result = suite.run::<Logger>(config).await;
			results.push(result);
		}
		results.into()
	}
}
