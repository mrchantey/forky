use super::*;
use anyhow::Result;
use forky_web::*;
use leptos::*;

pub struct TestRunnerWasm;

pub fn interactive_mode() -> bool { !SearchParams::get_flag("run") }

impl TestRunnerWasm {
	pub fn run() -> Result<()> {
		forky_web::set_panic_hook();
		if interactive_mode() {
			mount_to_body(|cx| view! {cx,<Root/>});
		} else {
			run_tests_wasm();
		}
		Ok(())
	}
}
