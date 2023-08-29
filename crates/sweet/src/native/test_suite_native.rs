use crate::*;
use anyhow::Error;
use rayon::prelude::*;
use tokio::task::JoinError;

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
		let to_run = TestCaseNative::split_funcs(to_run);
		// std::panic::set_hook(Box::new(|_| {}));
		// let _ = std::panic::take_hook();
		if config.parallel {
			let result = run_native_parallel(&to_run).await;
			match result {
				Ok(errors) => errors,
				Err(err) => panic!("\nUncaught error running tests in parallel. This shouldn't happen, please open an issue, for now run without the `--parallel` flag\n{:?}\n", err),
			}
		} else {
			run_native_series(&to_run).await
		}
	}
}

async fn run_native_series(to_run: &TestCaseNativeSplit<'_>) -> Vec<Error> {
	let mut errors = Vec::new();
	for (t, f) in to_run.syncs.iter() {
		let result = unwrap_panic(f);
		let result = t.format_error(result);
		if let Err(err) = result {
			errors.push(err);
		}
	}

	for (t, f) in to_run.series.iter() {
		let result = unwrap_panic_blocking(f);
		let result = t.format_error(result);
		if let Err(err) = result {
			errors.push(err);
		}
	}

	for (t, f) in to_run.parallels.iter() {
		let result = unwrap_panic_async(f()).await;
		let result = t.format_error(result);
		if let Err(err) = result {
			errors.push(err);
		}
	}
	errors
}
async fn run_native_parallel(
	to_run: &TestCaseNativeSplit<'_>,
) -> anyhow::Result<Vec<Error>> {
	if to_run.series.len() > 0 {
		panic!(
			"\n\nattempted to run suites containing 'nonSend' in parallel\n\n"
		)
	}

	let results_sync = to_run
		.syncs
		.par_iter()
		.map(|(t, f)| {
			let result = unwrap_panic(&f);
			t.format_error(result)
		});

	let handles_parallel = to_run.parallels.iter().map(|(t, f)| {
		let t = (*t).clone();
		let f = (*f).clone();
		tokio::spawn(async move {
			let result = unwrap_panic_async(f()).await;
			t.format_error(result)
		})
	});

	let results_parallel = futures::future::join_all(handles_parallel)
		.await
		.into_iter()
		.collect::<std::result::Result<Vec<_>, JoinError>>()?; //spawn error
	let errs = results_sync
		.chain(results_parallel)
		.filter_map(|r| r.err())
		.collect();
	Ok(errs)
}
