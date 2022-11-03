use bevy::prelude::*;
use forky_core::math::*;
use crate::*;
use super::*;

pub fn spawn_fps_camera(mut commands: Commands) {
	let translation = Vec3::new(0., 2.5, -5.0);
	let radius = translation.length();

	let child = commands
		.spawn_bundle(Camera3dBundle {
			transform: Transform::from_rotation(Quat::from_rotation_y(HALF_TAU)),
			..default()
		})
		.id();

	let parent = commands
		.spawn_bundle(SpatialBundle {
			transform: Transform::from_translation(translation)
				.with_rotation(Quat::from_rotation_x(TAU * 0.1)), // .looking_at(Vec3::ZERO, Vec3::Y)
			..default()
		})
		.insert(CameraParent)
		.insert(TransformController::default())
		.id();

	commands.entity(parent).push_children(&[child]);
}
