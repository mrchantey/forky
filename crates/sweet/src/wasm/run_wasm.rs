use super::*;
use crate::*;
use forky_core::wasm::*;
use leptos::*;
use std::time::Duration;


pub fn run_tests_wasm(collector: &TestCollectorWasm, config: &TestRunnerConfig) {
	let intro = TestRunner::pretty_print_intro(config);
	log!("{intro}");


	let start_time = performance_now();
	let results = collector.run(&config);
	let duration =
		Duration::from_millis((performance_now() - start_time) as u64);
	let summary = TestRunner::pretty_print_summary(&results, duration);

	log!("{summary}");
}