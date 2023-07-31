use super::*;
use crate::*;
use forky_core::*;
use forky_web::*;
use std::time::Duration;
use web_sys::console;


pub fn run_tests_wasm() {
	wasm_bindgen_futures::spawn_local(async move {
		forky_web::set_panic_hook();

		let config = TestRunnerConfig::from_search_params();
		let collector = TestCollectorWasm::new();

		let intro = TestRunner::pretty_print_intro(&config);
		console::clear();
		log!("{intro}");

		let start_time = performance_now();
		let results = collector.run(&config).await;
		let duration =
			Duration::from_millis((performance_now() - start_time) as u64);
		let summary = TestRunner::pretty_print_summary(&results, duration);

		log!("{summary}");
	});
}
