mod _matchers;
mod logging;
mod test_case;
pub use self::_matchers::*;
pub use forky_test::*;
pub use inventory;
pub use logging::*;
pub use test_case::*;

pub fn main() -> anyhow::Result<()> {
	#[cfg(target_arch = "wasm32")]
	{
		#[path = "wasm/mod.rs"]
		mod wasm;
		wasm::TestRunnerWasm::run()
	}
	#[cfg(not(target_arch = "wasm32"))]
	{
		#[path = "native/mod.rs"]
		mod native;
		native::TestRunnerNative::run()
	}
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
