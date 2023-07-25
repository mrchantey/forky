use crate::SuiteLogger;
use crate::*;
use anyhow::Error;

pub trait TestCollectorParallel<Case, Logger>:
	TestCollector<Case, Logger>
where
	Case: TestCase + Clone + Send + Sync,
	Logger: SuiteLogger + Send + Sync,
{
	async fn run_parallel(&self, config: &TestRunnerConfig) -> ResultSummary {
		if config.parallel {
			use futures::future::join_all;
			use rayon::prelude::*;
			let futs = self
				.suites_to_run(config)
				.par_iter()
				.map(move |s| {
					s.run::<Logger, TestSuiteRunnerParallel<Case>>(config)
				})
				.collect::<Vec<_>>();
			join_all(futs).await.into()
		} else {
			self.run(config).await
		}
	}
}


pub struct TestSuiteRunnerParallel<Case> {
	_case: std::marker::PhantomData<Case>,
}

impl<Case> TestSuiteRunner<Case> for TestSuiteRunnerParallel<Case>
where
	Case: TestCase + Clone + Send + Sync,
{
	async fn run_cases(to_run: Vec<&Case>, _: &TestRunnerConfig) -> Vec<Error> {
		use async_std::task::block_on;
		// use futures::future::join_all;
		use rayon::prelude::*;
		//TODO is it not possible to await join_all?
		to_run
			.par_iter()
			.map(move |t| block_on(t.run()))
			.filter_map(|result| result.err())
			.collect::<Vec<_>>()
	}
}
