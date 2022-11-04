use bevy::prelude::*;
use extend::ext;

use crate::utility;

#[ext]
pub impl Vec3 {
	fn add_x(mut self, x: f32) -> Self {
		self.x += x;
		self
	}
	fn add_y(mut self, y: f32) -> Self {
		self.y += y;
		self
	}
	fn add_z(mut self, z: f32) -> Self {
		self.z += z;
		self
	}
}
