use crate::*;
use bevy::prelude::*;
use web_sys::*;

pub struct Pose {
	pub position: Vec3,
	pub rotation: Quat,
}

impl Pose {
	// pub fn from(transform: &XrRigidTransform) -> Self {
	// }
}

#[rustfmt::skip]
impl From<XrRigidTransform> for Pose {
	fn from(transform: XrRigidTransform) -> Self {
		// let position = bevy_utils::dom_point_to_vec3_invert_x(
		// 	&transform.position());
		let position = bevy_utils::dom_point_to_vec3(
			&transform.position());
		let rotation = bevy_utils::dom_point_to_quat(
			&transform.orientation());
		// let rotation = bevy_utils::dom_point_to_quat_invert_yaw_roll(
		// 	&transform.orientation());
		Self { position, rotation }
	}
}
impl From<XrPose> for Pose {
	fn from(pose: XrPose) -> Self { Self::from(pose.transform()) }
}
