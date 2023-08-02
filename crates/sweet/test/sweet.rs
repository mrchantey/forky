#![feature(imported_main)]
pub use sweet::*;
mod runner;
#[cfg(target_arch = "wasm32")]
mod wasm;
