use forky_core::math::*;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
	let mut transform = Transform::from_xyz(0., 1., -10.);
	transform.rotate_local_y(TAU / 2.);
	commands.spawn().insert_bundle(Camera3dBundle {
		// transform::Transform
		transform,
		// transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}
