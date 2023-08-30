#![feature(imported_main, async_closure, async_fn_in_trait)]
pub use sweet::*;
mod common;
// #[cfg(not(target_arch = "wasm32"))]
mod native;
// #[cfg(target_arch = "wasm32")]
// mod wasm;
