use super::super::*;
use super::*;
use crate::astro::planets::ecliptic_positions::OrbitalElements;
use std::f64::consts::PI;

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
	pub fn new(right_ascention: f64, declination: f64, radius: f64) -> Self {
		Self {
			right_ascention,
			declination,
			radius,
		}
	}

	///converts longitude (deg) to right ascention (hours)
	pub fn from_spherical(spherical: &SphericalCoords) -> Self {
		Self {
			right_ascention: spherical.longitude * DEG2HOURS,
			declination: spherical.latitude,
			radius: spherical.radius,
		}
	}

	///coverts right ascention (hours) to longitude (deg)
	pub fn to_spherical(&self) -> SphericalCoords {
		SphericalCoords {
			longitude: self.right_ascention * HOURS2DEG,
			latitude: self.declination,
			radius: self.radius,
		}
	}

	pub fn oblate_latitude_correction(latitude: f64) -> f64 {
		latitude - 0.1924 * sin_d(2.0 * latitude)
	}
	pub fn oblate_radius_correction(latitude: f64) -> f64 {
		0.99833 + 0.00167 * cos_d(2.0 * latitude)
	}

	pub fn to_rectangular(&self) -> GeoCoords {
		GeoCoords::new(
			self.radius * cos_d(self.right_ascention) * cos_d(self.declination),
			self.radius * sin_d(self.right_ascention) * cos_d(self.declination),
			self.radius * sin_d(self.declination),
		)
	}
}


//TODO georect instead
impl GeoCoords {
	pub fn to_equatorial(
		&self,
		day: Y2000Day,
		position: &GeographicCoords,
	) -> EquatorialCoords {
		let dx = self.x;
		let dy = self.y;
		let dz = self.z;

		let k = day.obl_ecl2();
		let cos_k = cos_d(k);
		let sin_k = sin_d(k);

		let qx = dx;
		let qy = (dy * cos_k) - (dz * sin_k);
		let qz = (dy * sin_k) + (dz * cos_k);

		let eq = GeoCoords::new(qx, qy, qz).to_spherical().to_equatorial();

		let parallax = 1.0 / (EARTH_RADII_PER_AU * eq.radius);
		if parallax < PI / (180.0 * 3600.0) {
			return eq;
		}
		let parallax = parallax.asin();

		let gclat =
			EquatorialCoords::oblate_latitude_correction(position.latitude);
		let rho = EquatorialCoords::oblate_radius_correction(position.latitude);

		let ls = OrbitalElements::get_l(day, &ecliptic_positions::SUN);
		let gmst0 = (ls + 180.0) * DEG2HOURS;
		let utc_hour = day.utc_hour();
		let lst = gmst0 + utc_hour + position.longitude * DEG2HOURS;
		let hour_angle = lst - eq.right_ascention;
		let g = (tan_d(gclat) / cos_h(hour_angle)).atan() * RAD2DEG;

		let top_ra = eq.right_ascention
			- parallax * RAD2HOURS * rho * cos_d(gclat) * sin_h(hour_angle)
				/ cos_d(eq.declination);
		let top_dec = if g.abs() < 1.0e-6 {
			eq.declination
				- parallax
					* RAD2DEG * rho * sin_d(-eq.declination)
					* cos_h(hour_angle)
		} else {
			eq.declination
				- parallax
					* RAD2DEG * rho * sin_d(gclat)
					* sin_d(g - eq.declination)
					/ sin_d(g)
		};
		return EquatorialCoords::new(top_ra, top_dec, eq.radius);
	}
}

impl SphericalCoords {
	pub fn to_equatorial(&self) -> EquatorialCoords {
		EquatorialCoords::from_spherical(self)
	}
}
