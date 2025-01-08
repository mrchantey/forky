#![doc = include_str!("../README.md")]
#[cfg(feature = "bevy")]
pub use forky_bevy as bevy;
#[cfg(feature = "cli")]
pub use forky_cli as cli;
pub use forky_core as core;
#[cfg(feature = "fs")]
pub use forky_fs as fs;
#[cfg(feature = "net")]
pub use forky_net as net;
#[cfg(feature = "web")]
pub use forky_web as web;

pub mod prelude {
	#[cfg(feature = "bevy")]
	pub use crate::bevy::prelude::*;
	#[cfg(feature = "cli")]
	pub use crate::cli::prelude::*;
	pub use crate::core::prelude::*;
	#[cfg(feature = "fs")]
	pub use crate::fs::prelude::*;
	#[cfg(feature = "net")]
	pub use crate::net::prelude::*;
	#[cfg(feature = "web")]
	pub use crate::web::prelude::*;
}
