use crate::*;
use futures::future::join_all;

#[derive(Default, Debug, Clone)]
pub struct TestSuiteNative {
	pub file: String,
	pub tests: Vec<TestCaseNative>,
	pub config: TestSuiteConfig,
}


impl TestSuiteTrait<TestCaseNative> for TestSuiteNative {
	fn new(file: String) -> Self {
		Self {
			file,
			tests: Vec::new(),
			config: TestSuiteConfig::default(),
		}
	}
	fn file(&self) -> &str { self.file.as_str() }
	fn config(&self) -> &TestSuiteConfig { &self.config }
	fn tests(&self) -> &Vec<TestCaseNative> { &self.tests }
	fn tests_mut(&mut self) -> &mut Vec<TestCaseNative> { &mut self.tests }

	async fn run_cases(
		to_run: Vec<&TestCaseNative>,
		config: &TestRunnerConfig,
	) -> Vec<anyhow::Error> {
		if config.parallel {
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
		} else {
			run_cases_series::<TestCaseNative>(to_run, config).await
		}
	}
}
