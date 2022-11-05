use bevy::prelude::*;
use extend::ext;
use forky_core::*;
use std::ops::Add;

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

	fn random_on_sphere() -> Self {
		Vec3::new(random::value(), random::value(), random::value()) * 2. - 1.
	}
}
