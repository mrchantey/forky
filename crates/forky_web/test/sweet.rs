#![feature(imported_main)]
pub use sweet::*;
#[cfg(target_arch = "wasm32")]
#[path = "./mod.rs"]
mod tests;
