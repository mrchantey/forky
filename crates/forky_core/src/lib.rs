#![feature(async_closure)]
pub mod extensions;
pub mod forky_event;
pub mod graph;
pub mod macros;
pub mod math;
pub mod math_f64;
pub mod net;
pub mod utils;


pub mod prelude {
	pub use crate::extensions::*;
	pub use crate::forky_event::*;
	pub use crate::graph::*;
	pub use crate::macros::*;
	pub use crate::net::*;
	pub use crate::utils::*;
}
