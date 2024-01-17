use bevy_math::prelude::*;
use extend::ext;
use forky_core::prelude::*;

#[ext]
pub impl Vec3 {
	const RIGHT: Vec3 = Vec3::new(1., 0., 0.);
	const LEFT: Vec3 = Vec3::new(-1., 0., 0.);
	const UP: Vec3 = Vec3::new(0., 1., 0.);
	const DOWN: Vec3 = Vec3::new(0., -1., 0.);
	const Z: Vec3 = Vec3::new(0., 0., 1.);
	const Z_NEG: Vec3 = Vec3::new(0., 0., -1.);

	fn from_x(x: f32) -> Self { Vec3::new(x, 0., 0.) }
	fn from_y(y: f32) -> Self { Vec3::new(0., y, 0.) }
	fn from_z(z: f32) -> Self { Vec3::new(0., 0., z) }
	fn add_x(&mut self, x: f32) -> &mut Self {
		self.x += x;
		self
	}
	fn add_y(&mut self, y: f32) -> &mut Self {
		self.y += y;
		self
	}
	fn add_z(&mut self, z: f32) -> &mut Self {
		self.z += z;
		self
	}
	fn swap_xy(&mut self) -> &mut Self {
		let tmp = self.x;
		self.x = self.y;
		self.y = tmp;
		self
	}
	fn swap_xz(&mut self) -> &mut Self {
		let tmp = self.x;
		self.x = self.z;
		self.z = tmp;
		self
	}
	fn swap_yz(&mut self) -> &mut Self {
		let tmp = self.z;
		self.z = self.y;
		self.y = tmp;
		self
	}
	fn negate_x(&mut self) -> &mut Self {
		self.x = -self.x;
		self
	}
	fn negate_y(&mut self) -> &mut Self {
		self.y = -self.y;
		self
	}
	fn negate_z(&mut self) -> &mut Self {
		self.z = -self.z;
		self
	}


	fn random_on_sphere() -> Self {
		Vec3::new(random_value(), random_value(), random_value()) * 2. - 1.
	}
}
