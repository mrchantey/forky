use super::*;
use crate::SuiteLogger;
use futures::future::join_all;
use std::collections::HashMap;


pub trait TestCollector<Case, Logger>
where
	Case: TestCase + Clone + Sized,
	Logger: SuiteLogger,
{
	fn suites(&self) -> &Vec<TestSuite<Case>>;
	fn suites_to_run(
		&self,
		config: &TestRunnerConfig,
	) -> Vec<&TestSuite<Case>> {
		self.suites()
			.iter()
			.filter(|s| config.suite_passes_filter(s.file.as_str()))
			.collect::<Vec<_>>()
	}

	fn collect_cases() -> Vec<Case>;

	fn collect_suites() -> Vec<TestSuite<Case>> {
		let mut files: HashMap<String, TestSuite<Case>> = HashMap::new();
		let cases = Self::collect_cases();
		for case in cases.iter() {
			// let case: &T = &case;
			if !files.contains_key(case.file()) {
				let file = String::from(case.file());
				files.insert(file.clone(), TestSuite::new(file));
			}
			files.get_mut(case.file()).unwrap().tests.push(case.clone());
		}

		let mut files = files.iter().map(|f| f.1.clone()).collect::<Vec<_>>();
		files.sort_by(|a, b| a.file.cmp(&b.file));

		for file in files.iter_mut() {
			file.contains_only = file
				.tests
				.iter()
				.any(|t| *t.config() == TestCaseConfig::Only);
		}

		files
	}

	async fn run(&self, config: &TestRunnerConfig) -> ResultSummary {
		let to_run = self.suites_to_run(config);
		let mut results = Vec::with_capacity(to_run.len());
		for suite in to_run {
			let result = suite
				.run::<Logger>(config, TestSuite::<Case>::run_strategy)
				.await;
			results.push(result);
		}
		results.into()
	}
}


pub trait TestCollectorParallel<Case, Logger>:
	TestCollector<Case, Logger>
where
	Case: TestCase + Clone + Send + Sync,
	Logger: SuiteLogger,
{
	async fn run_parallel(&self, config: &TestRunnerConfig) -> ResultSummary {
		if config.parallel {
			use rayon::prelude::*;
			let futures = self
				.suites_to_run(config)
				.par_iter()
				.map(move |s| {
					s.run::<Logger>(
						config,
						TestSuite::<Case>::run_parallel_strategy,
					)
				})
				.collect::<Vec<_>>();
			join_all(futures).await.into()
		} else {
			self.run(config).await
		}
	}
}
