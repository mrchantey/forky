use bevy::prelude::*;
use extend::ext;
use forky_core::utility::random_value;

#[ext]
pub impl Vec3 {
	const RIGHT: Vec3 = Vec3::new(1., 0., 0.);
	fn from_x(x: f32) -> Self { Vec3::new(x, 0., 0.) }
	fn from_y(y: f32) -> Self { Vec3::new(0., y, 0.) }
	fn from_z(z: f32) -> Self { Vec3::new(0., 0., z) }
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
		Vec3::new(random_value(), random_value(), random_value()) * 2. - 1.
	}
}
