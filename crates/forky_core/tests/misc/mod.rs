#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod bevy_test;
pub use bevy_test::*;
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
