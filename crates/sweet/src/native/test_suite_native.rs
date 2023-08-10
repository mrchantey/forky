use crate::*;

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
		&self,
		to_run: Vec<&TestCaseNative>,
		config: &TestRunnerConfig,
	) -> Vec<anyhow::Error> {
		
		//TODO run manually and format results etc here
		
		// let to_run = to_run.iter().map(|case|case.func);
		// let (syncs, series, parallels) = Self::split_cases(to_run);
		// 	pub async fn run_funcs(funcs:impl IntoIterator<Item=&Self>) -> Result<()> {
		// 		let (syncs, series, parallels) = Self::split_funcs(funcs);
		// 		for f in syncs {
		// 			f()?;
		// 		}
		// 		for f in series {
		// 			f().await?;
		// 		}
		// 		for f in parallels {
		// 			f().await?;
		// 		}
		// 		Ok(())
		// 	}

		if config.parallel {
			run_cases_parallel(to_run, config).await
		} else {
			run_cases_series(to_run).await
		}
	}
}
