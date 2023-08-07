use super::*;
use crate::*;
use forky_core::*;
use forky_web::*;
use std::time::Duration;
use web_sys::console;

pub struct TestRunnerWasm;

impl TestRunnerWasm {
	pub fn run(config: &TestRunnerConfig) {
		let config = config.clone();
		wasm_bindgen_futures::spawn_local(async move {
			forky_web::set_panic_hook();

			let collector = TestCollectorWasm::new();

			let intro = TestRunner::pretty_print_intro(&config);
			console::clear();
			log!("{intro}");

			let start_time = performance_now();
			let to_run = collector.suites_to_run(&config);
			let results = TestRunner::run_group_series::<
				SuiteLoggerWasm,
				TestCaseWasm,
			>(to_run, &config)
			.await;
			let duration =
				Duration::from_millis((performance_now() - start_time) as u64);
			let summary = TestRunner::pretty_print_summary(&results, duration);

			log!("{summary}");
		});
	}
}
