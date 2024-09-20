#![allow(async_fn_in_trait)]
pub use forky_web_macros::*;
mod dom_utils;
pub use self::dom_utils::*;
#[cfg(feature = "leptos")]
mod dom_utils_leptos;
#[cfg(feature = "leptos")]
pub use self::dom_utils_leptos::*;
mod logging;
pub use self::logging::*;
mod extensions;
pub use self::extensions::*;
mod net;
pub use self::net::*;


pub mod prelude {
	pub use crate::dom_utils::*;
	#[cfg(feature = "leptos")]
	pub use crate::dom_utils_leptos::*;
	pub use crate::extensions::*;
	pub use crate::logging::*;
	pub use crate::net::*;
	pub use forky_web_macros::*;
}
