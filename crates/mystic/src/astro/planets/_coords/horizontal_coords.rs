use super::super::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HorizontalCoords {
	// degrees
	pub azimuth: f64,
	// degrees
	pub altitude: f64,
	pub distance: f64,
}

impl HorizontalCoords {
	pub fn new(azimuth: f64, altitude: f64, distance: f64) -> Self {
		Self {
			azimuth,
			altitude,
			distance,
		}
	}

	pub fn to_rectangular(&self) -> RectCoords {
		RectCoords::new(
			self.distance * cos_d(self.azimuth) * cos_d(self.altitude),
			self.distance * sin_d(self.azimuth) * cos_d(self.altitude),
			self.distance * sin_d(self.altitude),
		)
	}

	pub fn from_rectangular(rect: &RectCoords) -> HorizontalCoords {
		HorizontalCoords {
			distance: rect.length(),
			azimuth: wrap_deg(atan2_d(rect.y, rect.x)),
			altitude: atan2_d(rect.z, rect.length_xy()),
		}
	}


	pub fn from_equatorial_with_correction(
		day: Y2000Day,
		position: &GeographicCoords,
		equatorial: &EquatorialCoords,
	) -> Self {
		let horizon_correction_in_arc_minutes = -34.0;
		let hour_angle =
			day.hour_angle_st(position.longitude, equatorial.right_ascention);

		let sin_lat = sin_d(position.latitude);
		let cos_lat = cos_d(position.latitude);

		let sin_ha = sin_h(hour_angle);
		let cos_ha = cos_h(hour_angle);

		let sin_dec = sin_d(equatorial.declination);
		let cos_dec = cos_d(equatorial.declination);

		let altitude_ratio = (sin_lat * sin_dec) + (cos_lat * cos_dec * cos_ha);
		let abs_altitude_ratio = altitude_ratio.abs();
		if abs_altitude_ratio > 1.0 {
			if abs_altitude_ratio > 1.000001 {
				panic!("Internal error: altitude would be a complex number!");
			} else {
				// Just correct for apparent roundoff without going bezerk.
				return HorizontalCoords {
					altitude: if altitude_ratio < 0. { -90.0 } else { 90.0 },
					azimuth: 180.,
					distance: equatorial.radius,
				};
			}
		} else {
			let mut altitude = altitude_ratio.asin() * RAD2DEG;
			let abs_altitude = altitude.abs();
			let angle_correction_window = 6.0;
			//atmospheric lensing correction
			if abs_altitude < angle_correction_window
				&& horizon_correction_in_arc_minutes != 0.0
			{
				let linear_factor = (angle_correction_window - abs_altitude)
					/ angle_correction_window;
				altitude -=
					(horizon_correction_in_arc_minutes / 60.0) * linear_factor;
			}
			HorizontalCoords {
				azimuth: wrap_deg(atan2_d(
					-cos_dec * sin_ha,
					(cos_lat * sin_dec) - (sin_lat * cos_dec * cos_ha),
				)),
				altitude,
				distance: equatorial.radius,
			}
		}
	}

	#[rustfmt::skip]
	pub fn from_equatorial(
		day: Y2000Day,
		position: &GeographicCoords,
		equatorial: &EquatorialCoords,
	) -> HorizontalCoords {
		let ha = day.hour_angle_st(position.longitude, equatorial.right_ascention) * HOURS2DEG;

		let sin_lat = sin_d(position.latitude);
		let cos_lat = cos_d(position.latitude);

		let sin_dec = sin_d(equatorial.declination);
		let cos_dec = cos_d(equatorial.declination);

		let sin_ha = sin_d(ha);
		let cos_ha = cos_d(ha);

		let x_sid = cos_ha * cos_dec;
		let y_sid = sin_ha * cos_dec;
		let z_sid = sin_dec;

		let x_hor = x_sid * sin_lat - z_sid * cos_lat;
		let y_hor = y_sid;
		let z_hor = x_sid * cos_lat + z_sid * sin_lat;

		let azimuth = atan2_d(y_hor, x_hor) + 180.;
		let altitude = atan2_d(z_hor, f64::sqrt(x_hor * x_hor + y_hor * y_hor));

		HorizontalCoords {
			azimuth,
			altitude,
			distance: equatorial.radius,
		}
	}

	pub fn to_equatorial(
		&self,
		day: Y2000Day,
		position: &GeographicCoords,
	) -> EquatorialCoords {
		let sin_lat = sin_d(position.latitude);
		let cos_lat = cos_d(position.latitude);

		let sin_alt = sin_d(self.altitude);
		let cos_alt = cos_d(self.altitude);

		let ha = (self.azimuth - 180.) * DEG2HOURS;
		let sin_ha = sin_h(ha);
		let cos_ha = cos_h(ha);

		let x_hor = cos_alt * cos_ha;
		let y_hor = cos_alt * sin_ha;
		let z_hor = sin_alt;

		let x_sid = x_hor * sin_lat + z_hor * cos_lat;
		let y_sid = y_hor;
		let z_sid = -x_hor * cos_lat + z_hor * sin_lat;

		let lst = day.lst(position.longitude);

		let right_ascention = wrap_hours(lst - y_sid.atan2(x_sid) * RAD2HOURS);
		let declination = z_sid.asin() * RAD2DEG;

		EquatorialCoords {
			right_ascention,
			declination,
			radius: self.distance,
		}
	}
}

impl std::fmt::Display for HorizontalCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"azimuth: {:.prec$}, altitude: {:.prec$}",
			self.azimuth,
			self.altitude,
			prec = DISPLAY_PRECISION
		)
	}
}

impl RectCoords {
	pub fn to_horizontal(&self) -> HorizontalCoords {
		HorizontalCoords::from_rectangular(self)
	}
}

impl EquatorialCoords {
	pub fn to_horizontal(
		&self,
		day: Y2000Day,
		position: &GeographicCoords,
	) -> HorizontalCoords {
		HorizontalCoords::from_equatorial(day, position, self)
	}

	pub fn to_horizontal_with_correction(
		&self,
		day: Y2000Day,
		position: &GeographicCoords,
	) -> HorizontalCoords {
		HorizontalCoords::from_equatorial_with_correction(day, position, self)
	}

	pub fn from_horizontal(
		day: Y2000Day,
		position: &GeographicCoords,
		horizontal: &HorizontalCoords,
	) -> EquatorialCoords {
		horizontal.to_equatorial(day, position)
	}
}


// doesnt work, i think the above is correct
// function getMoonAzAlt(lon, lat, moon, utcHour, sidTime) {

// 	let sidTime_d = sidTime * hours2deg

// 	let gclat = lat - 0.1924 * sin_d(2 * lat)//sphere gclat = lat
// 	let rho = 0.99833 + 0.00167 * cos_d(2 * lat)//sphere rho = 1
// 	let mpar = Math.asin(1 / moon.r)

// 	let ra = moon.equatorialSphereGeo.rightAscention
// 	let dec = moon.equatorialSphereGeo.declination

// 	// let ha_geo = (sidTime - ra)
// 	// let ha_geo = (sidTime_d - ra)
// 	let ha_geo = (sidTime - ra) * hours2deg

// 	console.log(ha_geo);

// 	let g = atan_d(tan_d(gclat) / cos_d(ha_geo))

// 	let topRA = ra - mpar * rho * cos_d(gclat) * sin_d(ha_geo) / cos_d(dec)
// 	let topDec = dec - mpar * rho * sin_d(gclat) * sin_d(g - dec) / sin_d(g)

// 	// console.log(sidTime_d);
// 	let ha = sidTime_d - ra
// 	// console.dir(ha);
