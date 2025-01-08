#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]


// #![feature(never_type, never_type_fallback)]
#![feature(let_chains)]
pub mod auto_mod;
pub mod common;
pub mod key;
pub mod server;
pub mod style;
pub mod utils;
pub mod watch;

pub mod prelude {
	// pub use crate::auto_mod::*;
	pub use crate::common::*;
	pub use crate::utils::*;
	// pub use crate::server::*;
	// pub use crate::style::*;
	// pub use crate::watch::*;
}
