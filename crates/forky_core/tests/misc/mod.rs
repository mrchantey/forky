#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod bevy_test;
pub use bevy_test::*;
mod boid;
pub use boid::*;
mod macros;
pub use macros::*;
