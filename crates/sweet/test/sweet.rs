#![feature(imported_main, async_closure)]
pub use sweet::*;
mod common;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;
