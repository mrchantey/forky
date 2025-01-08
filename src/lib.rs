#![doc = include_str!("../README.md")]
pub use forky_core as core;
#[cfg(feature = "bevy")]
pub use forky_bevy as bevy;
#[cfg(feature = "cli")]
pub use forky_cli as cli;
#[cfg(feature = "fs")]
pub use forky_fs as fs;
#[cfg(feature = "server")]
pub use forky_server as server;
#[cfg(feature = "web")]
pub use forky_web as web;

pub mod prelude {
	pub use crate::core::prelude::*;
	#[cfg(feature = "bevy")]
	pub use crate::bevy::prelude::*;
	#[cfg(feature = "cli")]
	pub use crate::cli::prelude::*;
	#[cfg(feature = "fs")]
	pub use crate::fs::prelude::*;
	#[cfg(feature = "server")]
	pub use crate::server::prelude::*;
	#[cfg(feature = "web")]
	pub use crate::web::prelude::*;
}