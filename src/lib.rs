#![doc = include_str!("../README.md")]
pub use forky_core as core;
#[cfg(feature = "fs")]
pub use forky_fs as fs;
#[cfg(feature = "web")]
pub use forky_web as web;
#[cfg(feature = "cli")]
pub use forky_cli as cli;
#[cfg(feature = "bevy")]
pub use forky_bevy as bevy;

pub mod prelude {
	pub use crate::core::prelude::*;
	#[cfg(feature = "fs")]
	pub use crate::fs::prelude::*;
	#[cfg(feature = "web")]
	pub use crate::web::prelude::*;
	#[cfg(feature = "cli")]
	pub use crate::cli::prelude::*;
	#[cfg(feature = "bevy")]
	pub use crate::bevy::prelude::*;
}