#![cfg_attr(debug_assertions, allow(dead_code, unused_imports,unused_mut, unused_variables,unused_parens))]

mod generic;
pub use generic::*;
mod num_x;
pub use num_x::*;
mod path;
pub use path::*;
mod string;
pub use string::*;
mod str_x;
pub use str_x::*;
mod vec;
pub use vec::*;
