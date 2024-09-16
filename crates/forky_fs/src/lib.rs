#![feature(type_alias_impl_trait)]
#![feature(async_closure, async_fn_traits)]
pub mod cli;
pub mod utility;

pub mod prelude {
	pub use crate::cli::*;
	pub use crate::utility::*;
}
