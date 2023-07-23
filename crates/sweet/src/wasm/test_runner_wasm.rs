use std::time::Duration;
use super::*;
use crate::*;
use anyhow::Result;
use forky_core::wasm::*;
use leptos::*;

pub struct TestRunnerWasm;

impl TestRunnerWasm {
	pub fn run() -> Result<()> {
		forky_core::wasm::set_panic_hook();
		let config = TestRunnerConfig::from_search_params();

		let intro = TestRunner::pretty_print_intro(&config);
		log!("{intro}");
		
		let start_time = performance_now();

		let collector = TestCollectorWasm::new();
		let results = collector.run(&config);
		let duration = Duration::from_millis((performance_now() - start_time) as u64);
		let summary = TestRunner::pretty_print_summary(&results, duration);

		log!("{summary}");

		mount_to_body(|cx| view! {cx,<Root/>});
		Ok(())
	}
}
