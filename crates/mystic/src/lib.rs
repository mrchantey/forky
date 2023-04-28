// #![feature(async_fn_in_trait)]


pub mod astro;
#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
pub mod tarot;
mod utility;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
pub use utility::*;
