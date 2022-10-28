#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod _macros;
pub use _macros::*;
pub mod cli_args;
pub mod fs;
pub mod terminal;
