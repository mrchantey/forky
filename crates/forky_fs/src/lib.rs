#![feature(type_alias_impl_trait)]
#![cfg_attr(
	debug_assertions,
	allow(dead_code, unused_imports, unused_variables)
)]

mod utility;
pub use utility::*;
mod cli;
pub use cli::*;
