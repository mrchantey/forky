#![feature(imported_main, async_closure, async_fn_in_trait)]
pub use sweet::*;
mod common;
mod wasm;
#[cfg(feature = "bevy")]
mod bevy;