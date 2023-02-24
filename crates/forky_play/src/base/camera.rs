use bevy::prelude::*;
use forky_core::math::*;

pub fn spawn_camera(mut commands: Commands) {
	let mut transform = Transform::from_xyz(0., 1., -10.);
	transform.rotate_local_y(TAU / 2.);
	commands.spawn(Camera3dBundle {
		// transform::Transform
		transform,
		// transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}
