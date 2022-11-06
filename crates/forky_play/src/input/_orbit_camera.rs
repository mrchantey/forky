use super::*;
use crate::*;
use bevy::prelude::*;
use forky_core::math::*;

pub fn spawn_orbit_camera(mut commands: Commands) {
	let translation = Vec3::new(2., 5., 20.0);
	let radius = translation.length();

	let camera = commands
		.spawn_bundle(Camera3dBundle {
			// transform: Transform::from_rotation(Quat::from_rotation_y(
			// 	HALF_TAU,
			// )),
			transform: Transform::from_translation(translation)
				.looking_at(Vec3::ZERO, Vec3::Y),
			// .with_rotation(Quat::from_rotation_x(TAU * 0.1)),
			..default()
		})
		.insert(CameraParent)
		.insert(CameraViewType::Orbit)
		.insert(OrbitController {
			radius,
			..default()
		})
		.insert(TransformController::default());
}
