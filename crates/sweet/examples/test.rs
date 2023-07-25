#![feature(imported_main)]
#[path = "../test/sweet.rs"]
mod runner;
pub use runner::*;
