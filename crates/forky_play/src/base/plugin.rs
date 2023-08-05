use super::*;
use bevy::prelude::*;

pub struct BasicPlugin;

const CLEAR_COLOR: Color = Color::hsl(0.3 * 360., 1., 0.8);

impl Plugin for BasicPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(CLEAR_COLOR))
			.add_systems(Startup, spawn_basic_scene)
			.add_systems(Startup, spawn_cube)
			.add_systems(Startup, spawn_camera)
			.add_systems(Update, rotate_cube);
	}
}
