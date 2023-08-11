use crate::*;
use anyhow::Error;
use anyhow::Result;

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
		// std::panic::set_hook(Box::new(|_| {}));
		// let _ = std::panic::take_hook();
		if config.parallel {
			let result = run_native_parallel(to_run).await;
			match result {
				Ok(errors) => errors,
				Err(err) => panic!("\nUncaught error running tests in parallel. This shouldn't happen, please open an issue, for now run without the `--parallel` flag\n{:?}\n", err),
			}
		} else {
			run_native_series(to_run).await
		}
	}
}

async fn run_native_series(to_run: Vec<&TestCaseNative>) -> Vec<Error> {
	let (syncs, series, parallels) = TestCaseNative::split_funcs(to_run);
	let mut errors = Vec::new();
	for (t, f) in syncs.iter() {
		let result = unwrap_panic(f);
		let result = t.format_error(result);
		if let Err(err) = result {
			errors.push(err);
		}
	}

	for (t, f) in series.iter() {
		let result = unwrap_panic_blocking(f);
		let result = t.format_error(result);
		if let Err(err) = result {
			errors.push(err);
		}
	}

	for (t, f) in parallels.iter() {
		let result = unwrap_panic_async(f()).await;
		let result = t.format_error(result);
		if let Err(err) = result {
			errors.push(err);
		}
	}
	errors
}
async fn run_native_parallel(
	to_run: Vec<&TestCaseNative>,
) -> anyhow::Result<Vec<Error>> {
	let (syncs, series, parallels) = TestCaseNative::split_funcs(to_run);

	let handles_sync = syncs.iter().map(|(t, f)| {
		let t = (*t).clone();
		let f = (*f).clone();
		std::thread::spawn(move || {
			let result = unwrap_panic(&f);
			t.format_error(result)
		})
	});
	if series.len() > 0 {
		todo!("\n\nnon-paralllel tests run with parallel flag \n\n");
	}
	let handles_series = series.iter().map(|(t, f)| {
		let t = (*t).clone();
		let f = (*f).clone();
		tokio::spawn(async move {
			let result = unwrap_panic_blocking(&f);
			t.format_error(result)
		})
	});
	let handles_parallel = parallels.iter().map(|(t, f)| {
		let t = (*t).clone();
		let f = (*f).clone();
		tokio::spawn(async move {
			let result = unwrap_panic_async(f()).await;
			t.format_error(result)
		})
	});

	let results_sync = handles_sync.map(|h| anyhow_panic(h.join()));
	// let results_series = handles_series.map(|h| anyhow_panic(h.join()));
	let results_series = futures::future::join_all(handles_series)
		.await
		.into_iter()
		.map(|r| anyhow_tokio_join(r));
	let results_parallel = futures::future::join_all(handles_parallel)
		.await
		.into_iter()
		.map(|r| anyhow_tokio_join(r));
	let errs = results_sync
		.into_iter()
		.chain(results_series)
		.chain(results_parallel)
		.into_iter()
		.collect::<Result<Vec<anyhow::Result<_>>>>()? //spawn error
		.into_iter()
		.filter_map(|r| r.err())
		.collect();
	Ok(errs)
}
