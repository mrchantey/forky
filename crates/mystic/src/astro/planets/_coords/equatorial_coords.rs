use super::super::*;
use super::*;
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
	pub const VERNAL_EQUINOX: EquatorialCoords =
		EquatorialCoords::new(0., 0., 1.);
	pub const fn new(
		right_ascention: f64,
		declination: f64,
		radius: f64,
	) -> Self {
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

	pub fn to_rectangular(&self) -> RectCoords {
		RectCoords::new(
			self.radius
				* cos_d(self.right_ascention * HOURS2DEG)
				* cos_d(self.declination),
			self.radius
				* sin_d(self.right_ascention * HOURS2DEG)
				* cos_d(self.declination),
			self.radius * sin_d(self.declination),
		)
	}

	pub fn from_rectangular(rect: &RectCoords) -> EquatorialCoords {
		EquatorialCoords {
			radius: rect.length(),
			right_ascention: wrap_deg(atan2_d(rect.y, rect.x)) * DEG2HOURS,
			declination: atan2_d(rect.z, rect.length_xy()),
		}
	}

	//untested
	pub fn to_geo(&self, day: Y2000Day) -> GeoCoords {
		let rect = self.to_rectangular();
		let obl_ecl = day.obl_ecl();
		let cos_k = cos_d(-obl_ecl);
		let sin_k = sin_d(-obl_ecl);

		let x = rect.x;
		let y = rect.y * cos_k - rect.z * sin_k;
		let z = rect.y * sin_k + rect.z * cos_k;

		GeoCoords::new(x, y, z)
	}

	pub fn to_geo_with_correction(
		&self,
		day: Y2000Day,
		position: &GeographicCoords,
	) -> GeoCoords {
		let (ra, dec) = self.get_correction_offset(day, position);
		let mut eq = self.clone();
		eq.right_ascention += ra;
		eq.declination += dec;
		eq.to_geo(day)
	}

	pub fn from_geo(day: Y2000Day, geo: &GeoCoords) -> EquatorialCoords {
		let obl_ecl = day.obl_ecl();
		let cos_k = cos_d(obl_ecl);
		let sin_k = sin_d(obl_ecl);

		let x = geo.x;
		let y = geo.y * cos_k - geo.z * sin_k;
		let z = geo.y * sin_k + geo.z * cos_k;

		RectCoords::new(x, y, z).to_equatorial()
	}

	pub fn from_geo_with_correction(
		day: Y2000Day,
		position: &GeographicCoords,
		geo: &GeoCoords,
	) -> EquatorialCoords {
		let mut eq = geo.to_equatorial(day);
		let (ra, dec) = eq.get_correction_offset(day, position);
		eq.right_ascention -= ra;
		eq.declination -= dec;
		eq
	}

	pub fn get_correction_offset(
		&self,
		day: Y2000Day,
		position: &GeographicCoords,
	) -> (f64, f64) {
		let parallax = 1.0 / (self.radius * EARTH_RADII_PER_AU);
		if parallax < PI / (180.0 * 3600.0) {
			return (0., 0.);
		}
		let parallax = parallax.asin();

		// correct for oblateness/fatness fatness of earth at equator
		let gclat =
			EquatorialCoords::oblate_latitude_correction(position.latitude);
		let rho = EquatorialCoords::oblate_radius_correction(position.latitude);

		let hour_angle =
			day.hour_angle(position.longitude, self.right_ascention);
		let g = (tan_d(gclat) / cos_h(hour_angle)).atan() * RAD2DEG;

		let top_ra =
			parallax * RAD2HOURS * rho * cos_d(gclat) * sin_h(hour_angle)
				/ cos_d(self.declination);
		let top_dec = if g.abs() < 1.0e-6 {
			parallax
				* RAD2DEG * rho * sin_d(-self.declination)
				* cos_h(hour_angle)
		} else {
			parallax
				* RAD2DEG * rho * sin_d(gclat)
				* sin_d(g - self.declination)
				/ sin_d(g)
		};
		(top_ra, top_dec)
	}
}

impl std::fmt::Display for EquatorialCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"ra: {:.prec$}, dec: {:.prec$}, dist: {:.prec$}",
			self.right_ascention,
			self.declination,
			self.radius,
			prec = DISPLAY_PRECISION
		)
	}
}

impl SphericalCoords {
	pub fn to_equatorial(&self) -> EquatorialCoords {
		EquatorialCoords::from_spherical(self)
	}
}
impl RectCoords {
	pub fn to_equatorial(&self) -> EquatorialCoords {
		EquatorialCoords::from_rectangular(self)
	}
}

impl GeoCoords {
	pub fn to_equatorial(&self, day: Y2000Day) -> EquatorialCoords {
		EquatorialCoords::from_geo(day, self)
	}

	pub fn to_equatorial_with_correction(
		&self,
		day: Y2000Day,
		position: &GeographicCoords,
	) -> EquatorialCoords {
		EquatorialCoords::from_geo_with_correction(day, position, self)
	}
}
