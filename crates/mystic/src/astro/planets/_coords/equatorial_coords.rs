use super::super::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EquatorialCoords {
	///hours
	pub right_ascention: f64,
	///degrees
	pub declination: f64,
	///meters
	pub radius: f64,
}


impl EquatorialCoords {
	pub fn to_rectangular(&self) -> RectangluarCoords {
		RectangluarCoords {
			x: self.radius
				* cos_d(self.right_ascention)
				* cos_d(self.declination),
			y: self.radius
				* sin_d(self.right_ascention)
				* cos_d(self.declination),
			z: self.radius * sin_d(self.declination),
		}
	}
}
