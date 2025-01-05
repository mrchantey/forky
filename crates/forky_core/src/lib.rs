#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
#![feature(async_closure)]
pub mod extensions;
pub mod forky_event;
pub mod graph;
pub mod math;
pub mod math_f64;
pub mod utils;


pub mod prelude {
	pub use crate::extensions::*;
	pub use crate::forky_event::*;
	pub use crate::graph::*;
	pub use crate::math;
	pub use crate::math_f64;
	pub use crate::utils::*;
}
