#![feature(imported_main, async_closure)]
pub use sweet::*;
//TODO get common tests to run on wasm
// #[cfg(not(target_arch = "wasm32"))]
mod common;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;
