use bevy::prelude::*;
use extend::ext;

use crate::*;

#[ext]
pub impl Transform {
	fn flat_y(&self) -> Vec3 { Vec3::Y }
	fn flat_x(&self) -> Vec3 {
		let mut vec = self.local_x();
		vec.y = 0.;
		vec.normalize_or_zero()
	}
	fn flat_z(&self) -> Vec3 {
		let mut vec = self.local_z();
		vec.y = 0.;
		vec.normalize_or_zero()
	}

	fn from_rotation_x(x: f32) -> Self { Self::from_rotation(Quat::from_rotation_x(x)) }
	fn from_rotation_y(y: f32) -> Self { Self::from_rotation(Quat::from_rotation_y(y)) }
	fn from_rotation_z(z: f32) -> Self { Self::from_rotation(Quat::from_rotation_z(z)) }

	fn with_rotation_x(self, x: f32) -> Self { self.with_rotation(Quat::from_rotation_x(x)) }
	fn with_rotation_y(self, y: f32) -> Self { self.with_rotation(Quat::from_rotation_y(y)) }
	fn with_rotation_z(self, z: f32) -> Self { self.with_rotation(Quat::from_rotation_z(z)) }

	fn translate_x(&mut self, val: f32) { self.translation += self.local_x() * val; }
	fn translate_y(&mut self, val: f32) { self.translation += self.local_y() * val; }
	fn translate_z(&mut self, val: f32) { self.translation += self.local_z() * val; }
	fn translate_flat_x(&mut self, val: f32) { self.translation += self.flat_x() * val; }
	fn translate_flat_y(&mut self, val: f32) { self.translation += Vec3::Y * val; }
	fn translate_flat_z(&mut self, val: f32) { self.translation += self.flat_z() * val; }

	fn look_away(&mut self, target: Vec3, up: Vec3) {
		let forward = Vec3::normalize(target - self.translation);
		let right = up.cross(forward).normalize();
		let up = forward.cross(right);
		self.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
	}
	fn looking_away(mut self, target: Vec3, up: Vec3) -> Self {
		self.look_away(target, up);
		self
	}
	fn from_pose(&mut self, pose: &Pose) {
		self.translation = pose.position;
		self.rotation = pose.rotation;
	}
}
