#![cfg_attr(debug_assertions, allow(dead_code, unused_imports,unused_mut, unused_variables))]

pub mod char_shape;
pub mod generator;
pub mod square;
pub mod u8_shape;
mod _maze;
pub use _maze::*;
mod _plugin;
pub use _plugin::*;
mod _rect_maze;
pub use _rect_maze::*;
