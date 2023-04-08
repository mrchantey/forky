use super::*;
use crate::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct OrbitCameraBundle {
	camera: Camera3dBundle,
	parent: CameraParent,
	view_type: CameraViewType,
	controller: OrbitController,
	transform_controller: input::TransformController,
}

impl Default for OrbitCameraBundle {
	fn default() -> Self { Self::new(Vec3::new(2., 3., 5.)) }
}
impl OrbitCameraBundle {
	pub fn new(position: Vec3) -> Self {
		let radius = position.length();
		Self {
			camera: Camera3dBundle {
				transform: Transform::from_translation(position)
					.looking_at(Vec3::ZERO, Vec3::Y),
				..default()
			},
			parent: CameraParent,
			view_type: CameraViewType::Orbit,
			controller: OrbitController {
				radius,
				..default()
			},
			transform_controller: input::TransformController::default(),
		}
	}
}
