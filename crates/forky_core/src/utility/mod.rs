#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod macros;
pub use macros::*;
mod project_root;
pub use project_root::*;
mod terminal;
pub use terminal::*;
mod utility;
pub use utility::*;
