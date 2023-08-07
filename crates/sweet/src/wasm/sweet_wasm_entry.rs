use super::*;
use crate::TestRunnerConfig;
use anyhow::Result;
use forky_web::*;
use leptos::*;


pub fn interactive_mode() -> bool { !SearchParams::get_flag("run") }

pub fn sweet_wasm_entry() -> Result<()> {
	forky_web::set_panic_hook();
	if interactive_mode() {
		mount_to_body(|cx| view! {cx,<Root/>});
	} else {
		let config = TestRunnerConfig::from_search_params();
		TestRunnerWasm::run(&config);
	}
	Ok(())
}
