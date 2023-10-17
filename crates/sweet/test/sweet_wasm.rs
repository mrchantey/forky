#![feature(imported_main, async_closure)]
pub use sweet::*;
mod common;
mod wasm_tests;
#[cfg(feature = "bevy")]
mod bevy_tests;