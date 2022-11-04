#![cfg_attr(debug_assertions, allow(dead_code, unused_imports,unused_mut, unused_variables))]

pub mod cli_args;
pub mod file;
pub mod fs;
pub mod random;
pub mod terminal;
pub mod time;
mod _macros;
pub use _macros::*;
