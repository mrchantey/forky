use std::f64::consts::{PI, TAU};


pub const DISPLAY_PRECISION: usize = 6;


pub const DEG2RAD: f64 = PI / 180.;
pub const RAD2DEG: f64 = 180. / PI;
pub const RAD2HOURS: f64 = 12. / PI;
pub const HOURS2RAD: f64 = PI / 12.;
pub const DEG2HOURS: f64 = 1. / 15.;
pub const HOURS2DEG: f64 = 15.;

pub const METERS_PER_AU: f64 = 1.4959787e+11;
pub const METERS_PER_EARTH_RADII: f64 = 6378140.0;
pub const EARTH_RADII_PER_AU: f64 = METERS_PER_AU / METERS_PER_EARTH_RADII; // 23454.779920164812

pub fn sin_d(value: f64) -> f64 { f64::sin(value * DEG2RAD) }
pub fn cos_d(value: f64) -> f64 { f64::cos(value * DEG2RAD) }
pub fn tan_d(value: f64) -> f64 { f64::tan(value * DEG2RAD) }
pub fn asin_d(value: f64) -> f64 { f64::asin(value * DEG2RAD) }
pub fn acos_d(value: f64) -> f64 { f64::acos(value * DEG2RAD) }
pub fn atan_d(value: f64) -> f64 { f64::atan(value * DEG2RAD) }
pub fn atan2_d(x: f64, y: f64) -> f64 { f64::atan2(x, y) * RAD2DEG }

pub fn sin_h(value: f64) -> f64 { f64::sin(value * HOURS2RAD) }
pub fn cos_h(value: f64) -> f64 { f64::cos(value * HOURS2RAD) }
pub fn tan_h(value: f64) -> f64 { f64::tan(value * HOURS2RAD) }
pub fn asin_h(value: f64) -> f64 { f64::asin(value * HOURS2RAD) }
pub fn acos_h(value: f64) -> f64 { f64::acos(value * HOURS2RAD) }
pub fn atan_h(value: f64) -> f64 { f64::atan(value * HOURS2RAD) }
pub fn atan2_h(x: f64, y: f64) -> f64 { f64::atan2(x, y) * RAD2HOURS }


pub fn wrap_rad(value: f64) -> f64 { wrap(value, TAU) }
pub fn wrap_deg(value: f64) -> f64 { wrap(value, 360.) }
pub fn wrap_hours(value: f64) -> f64 { wrap(value, 24.) }


//TODO use forky_core wrap
pub fn wrap(value: f64, wrap: f64) -> f64 {
	value - f64::floor(value / wrap) * wrap
}

pub fn deg_min_sec(value: f64) -> (i64, i64, f64) {
	let deg = value.floor() as i64;
	let min = ((value - deg as f64) * 60.).floor() as i64;
	let sec = ((value - deg as f64 - min as f64 / 60.) * 3600.).floor() as f64;
	(deg, min, sec)
}
pub fn deg_min_sec_str(value: f64) -> String {
	let (deg, min, sec) = deg_min_sec(value);
	format!("{}° {}' {}\"", deg, min, sec)
}

//https://stackoverflow.com/questions/1878907/how-can-i-find-the-difference-between-two-angles
pub fn angle_between_d(a: f64, b: f64) -> f64 {
	wrap(a - b + 180., 360.) - 180.
}
pub fn unsigned_angle_between_d(a: f64, b: f64) -> f64 {
	f64::abs(angle_between_d(a, b))
}


// https://github.com/CruiserOne/Astrolog/blob/2676d1c63d22c66513c38f032a189e990ad2011e/general.cpp#L284
// Modulus function for floating point values, in which we bring the given
// parameter to within the range of 0 to 360.
// real Mod(real d)
// {
//   if (d >= rDegMax)         // In most cases, value is only slightly
//     d -= rDegMax;           // out of range, so can test for it and
//   else if (d < 0.0)         // avoid the more complicated arithmetic.
//     d += rDegMax;
//   if (d >= 0 && d < rDegMax)
//     return d;
//   return (d - RFloor(d/rDegMax)*rDegMax);
// }

// const earthTiltRad = earthTiltDeg * deg2rad
