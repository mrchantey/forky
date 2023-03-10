use super::*;
use crate::*;
use bevy::prelude::*;
pub struct DebugCameraPlugin;

impl Plugin for DebugCameraPlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.insert_resource(CameraViewType::Orbit)
			// .insert_resource(CameraViewType::Top)
			.add_startup_system(spawn_side_cameras)
			.add_startup_system(spawn_orbit_camera)
			.add_system(orbit_camera_controller)
			.add_system(orbit_keyboard_controller)
			.add_system(camera_view_toggle)
			.add_startup_system(
				run_camera_view_toggle.in_base_set(StartupSet::PostStartup),
			)
			// .add_system(mouse_controller)
			// .add_system(keyboard_controller)
			// .add_startup_system(spawn_orbit_camera)
			// .add_startup_system(spawn_fps_camera)
			.forky();
	}
}
