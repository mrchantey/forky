#![feature(async_closure)]
#![allow(async_fn_in_trait)]

mod matchers;
mod test_case;
pub use self::matchers::*;
pub use sweet_macros::*;
pub use test_case::*;
mod test_suite;
pub use test_suite::*;
mod test_runner;
pub use test_runner::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
#[cfg(feature = "bevy_ecs")]
mod bevy_ecs;
#[cfg(feature = "bevy_ecs")]
pub use bevy_ecs::*;
#[cfg(feature = "bevy")]
mod bevy;
#[cfg(feature = "bevy")]
pub use bevy::*;

pub mod exports {
	pub use anyhow::Result;
	#[cfg(not(target_arch = "wasm32"))]
	pub use futures::future::CatchUnwind;
	#[cfg(not(target_arch = "wasm32"))]
	pub use futures::FutureExt;
	pub use inventory;
	// #[cfg(target_arch = "wasm32")]
	// pub use js_sys;
	//is full exports like this bad form?
	#[cfg(target_arch = "wasm32")]
	pub use js_sys::*;
	pub use serde_json;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen::prelude::*;
	#[cfg(target_arch = "wasm32")]
	pub use wasm_bindgen_futures::future_to_promise;
}

#[cfg(target_arch = "wasm32")]
// pub fn main() -> anyhow::Result<()> { wasm::sweet_wasm_entry() }
pub fn main() -> anyhow::Result<()> { wasm::sweet_wasm_entry() }

#[cfg(not(target_arch = "wasm32"))]
// #[tokio::main]
pub fn main() -> anyhow::Result<()> {
	use forky_fs::*;
	RunTestsNativeCommand.run_with_cli_args()
}
