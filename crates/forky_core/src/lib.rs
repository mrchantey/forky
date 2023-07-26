#![feature(async_closure)]
mod extensions;
pub mod graph;
pub mod math;
pub mod math_f64;
pub mod utility;
mod utils;
pub use self::extensions::*;
mod forky_event;
pub use self::forky_event::*;
pub use self::utils::*;
pub mod net;
#[cfg(target_arch = "wasm32")]
pub mod wasm;