use super::*;
use crate::*;
use anyhow::Result;
use leptos::*;

pub struct TestRunnerWasm;

impl TestRunnerWasm {
	pub fn run() -> Result<()> {
		forky_core::wasm::set_panic_hook();

		mount_to_body(|cx| view! {cx,<Root/>});
		Ok(())
	}
}
