#![feature(async_fn_in_trait, async_closure)]
mod _matchers;
mod logging;
mod test_case;
pub use self::_matchers::*;
pub use forky_test::*;
pub use logging::*;
pub use test_case::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;


#[cfg(target_arch = "wasm32")]
pub fn main() -> anyhow::Result<()> { wasm::TestRunnerWasm::run() }

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
	native::TestRunnerNative::run().await
}


pub mod exports{
	pub use inventory;
	pub use anyhow::Result;
	//i guess pub use async_std bad for treeshake
	pub use async_std::task::block_on;
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
