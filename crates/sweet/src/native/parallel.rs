use crate::SuiteLogger;
use crate::*;

pub trait TestCollectorParallel<Case, Logger, Suite>:
	TestCollector<Case, Logger, Suite>
where
	Case: TestCase + Clone + Send + Sync,
	Logger: SuiteLogger + Clone + Send + Sync,
	Suite: TestSuiteTrait<Case> + Clone + Send + Sync + Sized,
{
	async fn run_parallel_maybe(
		&self,
		config: &TestRunnerConfig,
	) -> ResultSummary {
		if config.parallel {
			use futures::future::join_all;
			let suites_to_run = self.suites_to_run(config);

			let futs = suites_to_run
				.iter()
				.map(async move |s| s.run::<Logger>(config).await)
				.collect::<Vec<_>>();
			join_all(futs).await.into()
		} else {
			self.run(config).await
		}
	}
}
