use super::*;
use crate::*;
use anyhow::Result;
use leptos::*;


pub struct TestRunnerWasm;

impl TestRunnerWasm {
	pub fn run() -> Result<()> {
		forky_core::wasm::set_panic_hook();

		let config = TestRunnerConfig::from_search_params();

		let suites = TestSuiteCollection::new();
		let results_cases_arr = suites.run(&config);
		// results_cases_arr.to

		mount_to_body(|cx| view! {cx,<Root/>});
		Ok(())
	}
}
