use super::super::*;
use super::*;
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct EclipticCoords(pub SphericalCoords);


impl EclipticCoords {
	pub fn new(longitude: f64, latitude: f64, radius: f64) -> Self {
		Self(SphericalCoords::new(longitude, latitude, radius))
	}

	pub fn from_rect(rect: &HelioCoords) -> Self {
		EclipticCoords::new(
			rect.flat_angle() * RAD2DEG,
			rect.up_angle() * RAD2DEG,
			rect.length(),
		)
	}

	//atan(tan θ / cos ε )
	///midheaven
	pub fn meridian_intersect(
		day: Y2000Day,
		position: &GeographicCoords,
	) -> Self {
		let obl_ecl = day.obl_ecl();
		let lst = day.lst(position.longitude);
		let y = tan_h(lst);
		let x = cos_d(obl_ecl);
		let longitude = atan2_d(y, x);
		Self::new(longitude + 180., 0., 1.)
	}

	pub fn western_horizon_intersect(
		day: Y2000Day,
		position: &GeographicCoords,
	) -> Self {
		let obl_ecl = day.obl_ecl();
		let lst = day.lst(position.longitude);
		let y = -cos_h(lst);
		let x = sin_h(lst) * cos_d(obl_ecl)
			+ tan_d(position.latitude) * sin_d(obl_ecl);
		// let mut angle = (nom / denom).atan() * RAD2DEG;
		let longitude = atan2_d(y, x);

		//wikipedia says this is nessecary? (finding ascendant)
		// if (Ascendant < 180) then
		//     Ascendant = Ascendant + 180
		// else
		//     Ascendant = Ascendant - 180

		Self::new(longitude, 0., 1.)
	}

	pub fn eastern_horizon_intersect(
		day: Y2000Day,
		position: &GeographicCoords,
	) -> Self {
		let mut ecliptical = Self::western_horizon_intersect(day, position);
		ecliptical.longitude += 180.;
		ecliptical
	}

	pub fn ascendant_midheaven_time(
		day: Y2000Day,
		position: &GeographicCoords,
	) -> f64 {
		let asc = EclipticCoords::eastern_horizon_intersect(day, &position);
		let mc = EclipticCoords::meridian_intersect(day, &position);
		let asc = asc.to_equatorial();
		let mc = mc.to_equatorial();
		asc.right_ascention - mc.right_ascention
	}
}
//todo helio
impl HelioCoords {
	pub fn to_ecliptical(&self) -> EclipticCoords {
		EclipticCoords::from_rect(self)
	}
}

// i think these are wrong
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
