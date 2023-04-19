use super::super::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HorizontalCoords {
	pub azimuth: f64,
	pub altitude: f64,
}

impl HorizontalCoords {
	pub fn new(azimuth: f64, altitude: f64) -> Self {
		Self { azimuth, altitude }
	}
	//bad
	pub fn from_position(
		day: Y2000Day,
		position: &GeographicCoords,
		body: Planet,
	) -> Self {
		ecliptic_positions::ecliptic_position(day, body)
			.to_geo(day)
			.to_equatorial(day, position)
			.to_horizontal(day, position)
	}

	pub fn from_equatorial(
		day: Y2000Day,
		position: &GeographicCoords,
		equatorial: &EquatorialCoords,
	) -> Self {
		Self::from_equatorial_with_correction(day, position, equatorial, -34.0)
	}

	pub fn from_equatorial_with_correction(
		day: Y2000Day,
		position: &GeographicCoords,
		equatorial: &EquatorialCoords,
		horizon_correction_in_arc_minutes: f64,
	) -> Self {
		let gst = day.greenwich_sidereal_time_in_hours();
		let lst = gst + (position.longitude * DEG2HOURS);
		let hour_angle = wrap_hours(lst - equatorial.right_ascention);

		let sin_lat = sin_d(position.latitude);
		let cos_lat = cos_d(position.latitude);

		let sin_hour_angle = sin_h(hour_angle);
		let cos_hour_angle = cos_h(hour_angle);

		let sin_dec = sin_d(equatorial.declination);
		let cos_dec = cos_d(equatorial.declination);

		let altitude_ratio =
			(sin_lat * sin_dec) + (cos_lat * cos_dec * cos_hour_angle);
		let abs_altitude_ratio = altitude_ratio.abs();
		if abs_altitude_ratio > 1.0 {
			if abs_altitude_ratio > 1.000001 {
				panic!("Internal error: altitude would be a complex number!");
			} else {
				// Just correct for apparent roundoff without going bezerk.
				return HorizontalCoords {
					altitude: if altitude_ratio < 0. { -90.0 } else { 90.0 },
					azimuth: 180.,
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
					-cos_dec * sin_hour_angle,
					(cos_lat * sin_dec) - (sin_lat * cos_dec * cos_hour_angle),
				)),
				altitude,
			}
		}
	}


	pub fn from_equatorial2(
		day: Y2000Day,
		position: &GeographicCoords,
		equatorial: &EquatorialCoords,
	) -> HorizontalCoords {
		let lmst = day.lmst(position.longitude);

		let ha = (lmst - equatorial.right_ascention) * HOURS2DEG;

		let x_sid = cos_d(ha) * cos_d(equatorial.declination);
		let y_sid = sin_d(ha) * cos_d(equatorial.declination);
		let z_sid = sin_d(equatorial.declination);

		let x_hor =
			x_sid * sin_d(position.latitude) - z_sid * cos_d(position.latitude);
		let y_hor = y_sid;
		let z_hor =
			x_sid * cos_d(position.latitude) + z_sid * sin_d(position.latitude);

		let azimuth = atan2_d(y_hor, x_hor) + 180.;
		let altitude = atan2_d(z_hor, f64::sqrt(x_hor * x_hor + y_hor * y_hor));

		HorizontalCoords { altitude, azimuth }
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
