use super::*;
use anyhow::Result;
use leptos::*;

pub struct TestRunnerWasm;

impl TestRunnerWasm {
	pub fn run() -> Result<()> {
		forky_web::set_panic_hook();

		mount_to_body(|cx| view! {cx,<Root/>});
		Ok(())
	}
}
