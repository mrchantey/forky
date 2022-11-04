use bevy::prelude::*;
use extend::ext;
use forky_core::*;

use crate::utility;

#[ext]
pub impl Quat {
	fn forward(&self) -> Vec3 { *self * Vec3::Z }

	fn from_up() -> Quat { Quat::look_at(Vec3::Y) }
	fn from_down() -> Quat { Quat::look_at(-Vec3::Y) }
	fn from_right() -> Quat { Quat::look_at(Vec3::X) }
	fn from_left() -> Quat { Quat::look_at(-Vec3::X) }
	fn from_forward() -> Quat { Quat::look_at(Vec3::Z) }
	fn from_back() -> Quat { Quat::look_at(-Vec3::Z) }


	fn look_at(target: Vec3) -> Quat {
		let up = tern!(target.x == 0. && target.z == 0.; -Vec3::Z; Vec3::Y);
		let mat = Mat4::look_at_rh(target, Vec3::ZERO, up).inverse();
		Quat::from_mat4(&mat)
	}
}
