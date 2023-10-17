//! Declarative full-stack test framework.
//!
//! # Usage
//!
//! ```rust
//! // ./examples/sweet.rs
//!
//!	#![feature(imported_main)]
//! use sweet::{sweet_test, expect, Result};
//!
//! #[sweet_test]
//! fn it_passes() -> Result<()>{
//! 	expect(true).to_be_true()
//! }
//!
//! ```
//!
//! ```sh
//! cargo run --example sweet
//! ```
//!

#![feature(async_closure)]
#![allow(async_fn_in_trait)]
pub use sweet_macros::*;

pub mod matchers;
pub use matchers::expect;
pub use matchers::MatcherTExt;
pub mod test_case;
pub mod test_runner;
pub mod test_suite;
// pub use test_runner::*;

#[cfg(feature = "bevy_ecs")]
mod bevy_ecs;
#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(feature = "bevy_ecs")]
pub use bevy_ecs::*;
#[cfg(target_arch = "wasm32")]
pub use wasm::visit;
#[cfg(target_arch = "wasm32")]
pub use wasm::MatcherHtml;
#[cfg(feature = "bevy")]
mod bevy;
#[cfg(feature = "bevy")]
pub use bevy::*;

/// Re-exports for macros
pub mod exports {
	// pub use crate::test_case::TestCaseConfig;

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
pub fn main() -> anyhow::Result<()> { wasm::sweet_wasm_entry() }

#[cfg(not(target_arch = "wasm32"))]
// #[tokio::main]
pub fn main() -> anyhow::Result<()> {
	use forky_fs::*;
	native::RunTestsNativeCommand.run_with_cli_args()
}
