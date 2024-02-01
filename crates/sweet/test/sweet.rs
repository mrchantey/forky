#![feature(imported_main, async_closure)]
pub use sweet::main;
mod common;
// #[cfg(not(target_arch = "wasm32"))]
mod native_tests;
// #[cfg(target_arch = "wasm32")]
// mod wasm;
#[cfg(feature = "bevy")]
mod bevy_tests;
