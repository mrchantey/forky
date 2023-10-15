#![feature(imported_main, async_closure)]
pub use sweet::*;
mod common;
mod wasm;
#[cfg(feature = "bevy")]
mod bevy;