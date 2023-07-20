use anyhow::Result;

pub fn main() -> Result<()> {
	#[cfg(target_arch = "wasm32")]
	forky_core::wasm::set_panic_hook();
	#[cfg(not(target_arch = "wasm32"))]
	crate::native::TestRunnerNative::run()
	// TestRunner::run()
}
