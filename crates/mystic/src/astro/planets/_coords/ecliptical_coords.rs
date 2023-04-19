use super::super::*;
use super::*;
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct EclipticalCoords(pub SphericalCoords);


impl EclipticalCoords {
	pub fn new(longitude: f64, latitude: f64, radius: f64) -> Self {
		Self(SphericalCoords::new(longitude, latitude, radius))
	}

	pub fn from_rect(rect: &HelioCoords) -> Self {
		EclipticalCoords::new(
			rect.flat_angle() * RAD2DEG,
			rect.up_angle() * RAD2DEG,
			rect.length(),
		)
	}
}
//todo helio
impl HelioCoords {
	pub fn to_ecliptical(&self) -> EclipticalCoords {
		EclipticalCoords::from_rect(self)
	}
}


// pub fn ecliptical_to_equatorial(&self, obl_ecl: f64) -> RectCoords {
// 	RectCoords {
// 		x: self.x,
// 		y: self.y * cos_d(obl_ecl) - self.z * sin_d(obl_ecl),
// 		z: self.y * sin_d(obl_ecl) + self.z * cos_d(obl_ecl),
// 	}
// }

// pub fn equatorial_to_ecliptical(&self, obl_ecl: f64) -> RectCoords {
// 	RectCoords {
// 		x: self.x,
// 		y: self.y * cos_d(-obl_ecl) - self.z * sin_d(-obl_ecl),
// 		z: self.y * sin_d(-obl_ecl) + self.z * cos_d(-obl_ecl),
// 	}
// }
