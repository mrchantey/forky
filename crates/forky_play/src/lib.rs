#![feature(async_fn_in_trait)]
// #![allow(incomplete_features)]
// #![feature(return_position_impl_trait_in_trait)]

pub mod animation;
pub mod base;
pub mod camera;
pub mod debug;
pub mod input;
pub mod materials;
pub mod maze;
pub mod mesh_utils;
pub mod mithril;
pub mod physics;
pub mod plugins;
pub mod render_graph;
pub mod spline;
pub mod tool;
pub mod utility;

mod common;
pub use common::*;
mod extensions;
pub use extensions::*;
mod geometry__;
pub use geometry__::*;
