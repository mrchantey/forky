use super::*;
use crate::*;
use bevy::prelude::*;

pub fn spawn_side_cameras(mut commands: Commands) {
	let mut spawn = |position: Vec3, rotation: Quat, ct: CameraViewType| {
		commands
			.spawn_bundle(Camera3dBundle {
				transform: Transform::from_translation(position)
					.with_rotation(rotation),
				camera: Camera {
					is_active: false,
					..default()
				},
				..default()
			})
			.insert(ct);
	};

	spawn(
		Vec3::new(10., 0., 0.),
		Quat::from_right(),
		CameraViewType::Right,
	);
	spawn(Vec3::new(0., 10., 0.), Quat::from_up(), CameraViewType::Top);
	spawn(
		Vec3::new(0., 0., 10.),
		Quat::from_forward(),
		CameraViewType::Front,
	);
}
