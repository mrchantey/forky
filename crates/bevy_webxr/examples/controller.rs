use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_webxr::*;

fn main() {
	set_panic_hook();
	App::new()
		.add_plugin(bevy_utils::WebXrBasePlugin)
		.add_startup_system(demo::spawn_camera)
		.add_startup_system(demo::spawn_ground)
		.add_startup_system(demo::spawn_lights)
		.add_startup_system(spawn_controller)
		.run();
}

fn spawn_controller(mut commands: Commands, asset_server: Res<AssetServer>) {
	let handle: Handle<Scene> = asset_server.load("models/left.glb#Scene0");
	commands.spawn(SceneBundle {
		scene: handle,
		transform: Transform::from_xyz(0.0, 0.0, 0.0)
			.with_rotation(Quat::from_rotation_x(TAU * 0.25)),
		..default()
	});
}