mod _matchers;
mod logging;
mod test_case;
pub use self::_matchers::*;
pub use forky_test::*;
pub use inventory;
pub use logging::*;
pub use test_case::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub fn main() -> anyhow::Result<()> {
	#[cfg(target_arch = "wasm32")]
	forky_core::wasm::set_panic_hook();
	#[cfg(not(target_arch = "wasm32"))]
	crate::native::TestRunnerNative::run()
}


// fn sync_function() -> i32 {
// 	// Create a tokio runtime to run the async task
// 	let rt = Runtime::new().unwrap();

// 	// Block the synchronous function until the async function is complete
// 	let result = rt.block_on(async {
// 			async_operation().await
// 	});

// 	result
// }
