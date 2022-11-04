#![cfg_attr(debug_assertions, allow(dead_code, unused_imports,unused_mut, unused_variables))]

pub mod char_shape;
pub mod maze_3d;
pub mod maze_wall;
pub mod mesh_shape;
pub mod square;
pub mod u8_shape;
mod _depth_first_backtrack;
pub use _depth_first_backtrack::*;
mod _maze;
pub use _maze::*;
mod _plugin;
pub use _plugin::*;
mod _rect_maze;
pub use _rect_maze::*;
