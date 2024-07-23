use super::*;
use crate::*;
use bevy::prelude::*;


#[derive(Bundle)]
pub struct FlyCameraBundle {
	name: Name,
	camera: Camera3dBundle,
	view_type: CameraViewType,
	transform_controller: input::TransformController,
}


impl Default for FlyCameraBundle {
	fn default() -> Self { Self::forward() }
}

impl FlyCameraBundle {
	pub fn new(
		name: &'static str,
		transform: Transform,
		view_type: CameraViewType,
	) -> Self {
		Self {
			name: name.into(),
			camera: Camera3dBundle {
				transform,
				..default()
			},
			view_type,
			transform_controller: input::TransformController::default()
				.with_local_axis()
				.with_rotation_disabled()
				.clone(),
		}
	}

	pub fn right() -> Self {
		Self::new(
			"Right Camera",
			Transform::from_translation(Vec3::new(10., 0., 0.))
				.with_rotation(Quat::from_right()),
			CameraViewType::Right,
		)
	}
	pub fn forward() -> Self {
		Self::new(
			"Forward Camera",
			Transform::from_translation(Vec3::new(0., 0., 10.))
				.with_rotation(Quat::from_forward()),
			CameraViewType::Front,
		)
	}

	pub fn up() -> Self {
		Self::new(
			"Up Camera",
			Transform::from_translation(Vec3::new(0., 10., 0.))
				.with_rotation(Quat::from_up()),
			CameraViewType::Top,
		)
	}

	pub fn with_position(mut self, position: Vec3) -> Self {
		self.camera.transform.translation = position;
		self
	}

	pub fn as_disabled(mut self) -> Self {
		self.camera.camera.is_active = false;
		self
	}
}
