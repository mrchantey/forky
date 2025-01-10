#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
#![allow(async_fn_in_trait)]



#[cfg(feature = "axum")]
pub mod axum_utils;
#[cfg(feature = "axum")]
pub mod layers;
#[cfg(feature = "reqwest")]
pub mod reqwest_utils;
pub mod state;

pub mod prelude {
	#[cfg(feature = "axum")]
	pub use crate::axum_utils::*;
	#[cfg(feature = "axum")]
	pub use crate::layers::*;
	#[cfg(feature = "reqwest")]
	pub use crate::reqwest_utils::*;
	pub use crate::state::*;
}
