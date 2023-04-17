use super::super::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RectangluarCoords {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl RectangluarCoords {
	pub fn length(&self) -> f64 {
		f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
	}
	pub fn length_xy(&self) -> f64 {
		f64::sqrt(self.x * self.x + self.y * self.y)
	}

	pub fn to_equatorial(&self) -> EquatorialCoords {
		EquatorialCoords {
			radius: self.length(),
			right_ascention: f64::atan2(self.y, self.x) * rad2hours,
			declination: f64::atan2(self.z, self.length_xy()) * rad2deg,
		}
	}
	//yes identical to equatorial
	pub fn to_ecliptical(&self) -> EclipticalCoords {
		EclipticalCoords {
			radius: self.length(),
			longitude: f64::atan2(self.y, self.x) * rad2deg,
			latitude: f64::atan2(self.z, self.length_xy()) * rad2deg,
		}
	}

	pub fn ecliptical_to_equatorial(&self, obl_ecl: f64) -> RectangluarCoords {
		RectangluarCoords {
			x: self.x,
			y: self.y * cos_d(obl_ecl) - self.z * sin_d(obl_ecl),
			z: self.y * sin_d(obl_ecl) + self.z * cos_d(obl_ecl),
		}
	}

	pub fn equatorial_to_ecliptical(&self, obl_ecl: f64) -> RectangluarCoords {
		RectangluarCoords {
			x: self.x,
			y: self.y * cos_d(-obl_ecl) - self.z * sin_d(-obl_ecl),
			z: self.y * sin_d(-obl_ecl) + self.z * cos_d(-obl_ecl),
		}
	}
}
impl std::ops::Add for RectangluarCoords {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl std::ops::Sub for RectangluarCoords {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}
