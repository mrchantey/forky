#![feature(async_closure, let_chains)]
pub mod auto_mod;
pub mod common;
pub mod server;
pub mod style;
pub mod watch;

pub mod prelude {
	pub use crate::auto_mod::*;
	pub use crate::common::*;
	pub use crate::server::*;
	pub use crate::style::*;
	pub use crate::watch::*;
}
