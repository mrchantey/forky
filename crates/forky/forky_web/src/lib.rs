#![allow(async_fn_in_trait)]
pub use forky_web_macros::*;
mod dom_utils;
pub use self::dom_utils::*;
mod logging;
pub use self::logging::*;
mod ui;
pub use self::ui::*;
mod extensions;
pub use self::extensions::*;
mod net;
pub use self::net::*;


pub mod prelude {
	pub use crate::dom_utils::*;
	pub use crate::extensions::*;
	pub use crate::logging::*;
	pub use crate::net::*;
	pub use crate::ui::*;
	pub use forky_web_macros::*;
}
