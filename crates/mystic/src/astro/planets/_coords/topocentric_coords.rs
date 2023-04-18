use super::super::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TopocentricCoords {
	pub altitude: f64,
	pub azimuth: f64,
	pub longitude: f64,
	pub latitude: f64,
}


impl TopocentricCoords {
	// I THINK THIS SUN CASE CAN BE GENERAL CASE
	pub fn from_equatorial(
		equatorial: &EquatorialCoords,
		lat: f64,
		lon: f64,
		gmst: f64,
		utc_hour: f64,
	) -> TopocentricCoords {
		let lmst = gmst + utc_hour + lon; //lon in hours

		let ha = (lmst - equatorial.right_ascention) * HOURS2DEG;

		let x_sid = cos_d(ha) * cos_d(equatorial.declination);
		let y_sid = sin_d(ha) * cos_d(equatorial.declination);
		let z_sid = sin_d(equatorial.declination);

		let x_hor = x_sid * sin_d(lat) - z_sid * cos_d(lat);
		let y_hor = y_sid;
		let z_hor = x_sid * cos_d(lat) + z_sid * sin_d(lat);

		let azimuth = atan2_d(y_hor, x_hor) + 180.;
		let altitude = atan2_d(z_hor, f64::sqrt(x_hor * x_hor + y_hor * y_hor));

		TopocentricCoords {
			altitude,
			azimuth,
			longitude: lon,
			latitude: lat,
		}
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
