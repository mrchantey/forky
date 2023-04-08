use super::*;
use crate::*;
use bevy::prelude::*;
pub struct DebugCameraPlugin;

impl Plugin for DebugCameraPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(CameraViewType::Orbit)
			.add_startup_system(spawn_all_cameras)
			.add_system(orbit_camera_controller)
			.add_system(orbit_keyboard_controller)
			.add_system(camera_view_toggle)
			.add_startup_system(
				run_camera_view_toggle.in_base_set(StartupSet::PostStartup),
			)
			.__();
	}
}

fn spawn_all_cameras(mut commands: Commands) {
	commands.spawn(OrbitCameraBundle::default());
	commands.spawn(FlyCameraBundle::right().as_disabled());
	commands.spawn(FlyCameraBundle::forward().as_disabled());
	commands.spawn(FlyCameraBundle::up().as_disabled());
}
