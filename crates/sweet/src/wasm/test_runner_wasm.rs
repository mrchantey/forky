use super::*;
use crate::*;
use anyhow::Result;
use leptos::*;

pub struct TestRunnerWasm;

impl TestRunnerWasm {
	pub fn run() -> Result<()> {
		forky_core::wasm::set_panic_hook();
		collect_tests();

		// let config = TestRunnerConfig::from_search_params();

		// let suites = TestSuiteCollection::new();
		// let val = suites
		// 	.0
		// 	.iter()
		// 	.fold(String::new(), |s, t| s + t.file.to_string().as_str());
		// log!("suites: {}, tests: {}", suites.0.len(), val);
		// let results_cases_arr = suites.run(&config);
		// let result = results_cases_arr
		// 	.iter()
		// 	.fold(String::new(), |s, t| s + t.failed.to_string().as_str());
		// log!("result: {}", result);

		// results_cases_arr

		mount_to_body(|cx| view! {cx,<Root/>});
		Ok(())
	}
}
