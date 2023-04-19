use super::super::*;
use super::*;
use crate::astro::planets::ecliptic_positions::earth;
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct GeoCoords(pub RectCoords);

impl GeoCoords {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self(RectCoords::new(x, y, z))
	}
}

impl std::ops::Add for GeoCoords {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl std::ops::Sub for GeoCoords {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl From<RectCoords> for GeoCoords {
	fn from(other: RectCoords) -> Self { Self(other) }
}


impl HelioCoords {
	pub fn to_geo(&self, day: Y2000Day) -> GeoCoords {
		self.to_geo_with_earth(earth(day))
	}
	pub fn to_geo_with_earth(&self, pos_earth: HelioCoords) -> GeoCoords {
		GeoCoords::new(
			self.x - pos_earth.x,
			self.y - pos_earth.y,
			self.z - pos_earth.z,
		)
	}
}
