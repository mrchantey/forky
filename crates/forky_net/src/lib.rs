#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
#![allow(async_fn_in_trait)]



#[cfg(feature = "server")]
pub mod axum_utils;
#[cfg(feature = "server")]
pub mod layers;
pub mod state;
pub mod utils;

pub mod prelude {
	#[cfg(feature = "server")]
	pub use crate::axum_utils::*;
	#[cfg(feature = "server")]
	pub use crate::layers::*;
	pub use crate::state::*;
	pub use crate::utils::*;
}
