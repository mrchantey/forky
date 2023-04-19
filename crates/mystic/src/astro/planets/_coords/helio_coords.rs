use super::*;
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct HelioCoords(pub RectCoords);

impl HelioCoords {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self(RectCoords::new(x, y, z))
	}
}

impl std::ops::Add for HelioCoords {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl std::ops::Sub for HelioCoords {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}


impl From<RectCoords> for HelioCoords {
	fn from(other: RectCoords) -> Self {
		Self(other)
	}
}