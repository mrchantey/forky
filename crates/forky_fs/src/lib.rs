#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]


#![feature(type_alias_impl_trait)]
#![feature(async_fn_traits)]
pub mod cli;
pub mod utility;

pub mod prelude {
	pub use crate::cli::*;
	pub use crate::utility::*;
}
