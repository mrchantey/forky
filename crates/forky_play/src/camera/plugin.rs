use super::*;
use crate::*;
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_systems(Update, orbit_camera_controller)
			.add_systems(Update, orbit_keyboard_controller)
			.add_systems(Update, camera_view_toggle)
			.add_systems(PostStartup, toggle_startup_camera)
			.__();
	}
}

pub struct DebugCameraPlugin;

impl Plugin for DebugCameraPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(CameraViewType::Orbit)
			.add_systems(Startup, spawn_all_cameras)
			.__();
	}
}

fn spawn_all_cameras(mut commands: Commands) {
	commands.spawn(OrbitCameraBundle::default());
	commands.spawn(FlyCameraBundle::right());
	commands.spawn(FlyCameraBundle::forward());
	commands.spawn(FlyCameraBundle::up());
}
