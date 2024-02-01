use crate::native::*;
use crate::test_case::*;
use crate::test_runner::*;
use crate::test_suite::*;
use anyhow::Error;
use rayon::prelude::*;
use std::path::PathBuf;
use tokio::task::JoinError;

#[derive(Default, Debug, Clone)]
pub struct TestSuiteNative {
	pub file: PathBuf,
	pub tests: Vec<TestCaseNative>,
	pub config: TestSuiteConfig,
}


impl TestSuiteTrait<TestCaseNative> for TestSuiteNative {
	fn new(file: PathBuf) -> Self {
		Self {
			file,
			tests: Vec::new(),
			config: TestSuiteConfig::default(),
		}
	}
	fn file(&self) -> &PathBuf { &self.file }
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
			"\n\nattempted to run suites containing 'non_send' in parallel\n\n"
		)
	}


	let handles_parallel = to_run
		.parallels
		.iter()
		.map(|(t, f)| {
			let t = (*t).clone();
			let f = (*f).clone();
			tokio::spawn(async move {
				let result = unwrap_panic_async(f()).await;
				t.format_error(result)
			})
		})
		.collect::<Vec<_>>();

	let results_parallel = tokio::spawn(async move {
		futures::future::join_all(handles_parallel).await
	}); // TODO seems like awkward way to force handles to run

	let results_sync = to_run
		.syncs
		.par_iter()
		.map(|(t, f)| {
			let result = unwrap_panic(&f);
			t.format_error(result)
		})
		.collect::<Vec<_>>(); //blocks until syncs complete


	// let results_parallel = futures::future::join_all(handles_parallel).await
	let results_parallel = results_parallel
		.await? //blocks until parallels complete
		.into_iter()
		.collect::<std::result::Result<Vec<_>, JoinError>>()?;
	let errs = results_sync
		.into_iter()
		.chain(results_parallel)
		.filter_map(|r| r.err())
		.collect();
	Ok(errs)
}
