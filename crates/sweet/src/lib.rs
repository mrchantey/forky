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
// #[tokio::main]
pub fn main() -> anyhow::Result<()> { native::TestRunnerNative::run() }


pub mod exports {
	pub use anyhow::Result;
	#[cfg(not(target_arch = "wasm32"))]
	pub use async_std::task::block_on;
	#[cfg(not(target_arch = "wasm32"))]
	pub use futures::future::CatchUnwind;
	#[cfg(not(target_arch = "wasm32"))]
	pub use futures::FutureExt;
	pub use inventory;
	//is full exports like this bad form?
	// #[cfg(target_arch = "wasm32")]
	// pub use js_sys;
	#[cfg(target_arch = "wasm32")]
	pub use js_sys::*;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen::prelude::*;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen_futures::future_to_promise;
}
