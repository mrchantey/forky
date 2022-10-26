#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod bevy;
pub use bevy::*;
mod boid;
pub use boid::*;
mod custom;
pub use custom::*;
mod describe;
pub use describe::*;
mod log;
pub use log::*;
mod test;
pub use test::*;
mod test_runner;
pub use test_runner::*;
