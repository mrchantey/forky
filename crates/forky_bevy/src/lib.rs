#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

pub mod extensions;
pub mod systems;


pub mod prelude {
	pub use crate::extensions::*;
	pub use crate::systems::*;
}
