#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod basic_plugin;
pub use basic_plugin::*;
mod exit;
pub use exit::*;
mod keycodes;
pub use keycodes::*;
mod tag_components;
pub use tag_components::*;
mod test;
pub use test::*;
