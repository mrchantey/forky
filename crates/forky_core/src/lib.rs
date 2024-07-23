#![feature(async_closure)]
mod extensions;
pub mod graph;
pub mod math;
pub mod math_f64;
pub mod utility;
mod utils;
pub use self::extensions::*;
pub mod forky_event;
pub use self::forky_event::*;
pub use self::utils::*;
pub mod macros;
// pub use self::macros::*;
pub mod net;


pub mod prelude {
	pub use crate::extensions::*;
	pub use crate::forky_event::*;
	pub use crate::graph::*;
	pub use crate::macros::*;
	// pub use crate::math::*;
	// pub use crate::math_f64::*;
	pub use crate::net::*;
	pub use crate::utility::*;
	pub use crate::utils::*;
}
