use crate::*;
use bevy::prelude::*;
use extend::ext;

#[ext(name = OptI32X)]
pub impl App {
	fn run_webxr(mut self) { bevy_xr_utils::run_bevy_webxr(self); }
}
