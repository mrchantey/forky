use super::*;
use crate::*;
use bevy::{input::mouse::*, prelude::*, render::camera::*};
use forky_core::math::*;
pub struct DebugCameraPlugin;

impl Plugin for DebugCameraPlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.add_system(orbit_camera_controller)
			.add_system(orbit_keyboard_controller)
			// .add_system(camera_toggle)
			// .add_system(mouse_controller)
			// .add_system(keyboard_controller)
			// .add_startup_system(spawn_orbit_camera)
			// .add_startup_system(spawn_fps_camera)
			.add_startup_system(spawn_orbit_camera)
			.forky();
	}
}
