use super::*;
use crate::*;
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_system(orbit_camera_controller)
			.add_system(orbit_keyboard_controller)
			.add_system(camera_view_toggle)
			.add_startup_system(
				toggle_startup_camera.in_base_set(StartupSet::PostStartup),
			)
			.__();
	}
}

pub struct DebugCameraPlugin;

impl Plugin for DebugCameraPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(CameraViewType::Orbit)
			.add_startup_system(spawn_all_cameras)
			.__();
	}
}

fn spawn_all_cameras(mut commands: Commands) {
	commands.spawn(OrbitCameraBundle::default());
	commands.spawn(FlyCameraBundle::right());
	commands.spawn(FlyCameraBundle::forward());
	commands.spawn(FlyCameraBundle::up());
}
