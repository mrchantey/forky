use std::{
	f64::consts::PI,
	time::{Duration, SystemTime, UNIX_EPOCH},
};



pub const deg2rad: f64 = PI / 180.;
pub const rad2deg: f64 = 180. / PI;
pub const rad2hours: f64 = 12. / PI;
pub const hours2rad: f64 = PI / 12.;
pub const deg2hours: f64 = 1. / 15.;
pub const hours2deg: f64 = 15.;

// pub const earthTiltDeg:f64 = 23.4393;
pub const earthTiltDeg: f64 = 23.43656;
pub const metersPerAU: f64 = 1.4959787e+11;
pub const metersPerEarthRadii: f64 = 6378140.0;
pub const earthRadiiPerAU: f64 = metersPerAU / metersPerEarthRadii; // 23454.779920164812

pub fn sin_d(value: f64) -> f64 { f64::sin(value * deg2rad) }
pub fn cos_d(value: f64) -> f64 { f64::cos(value * deg2rad) }
pub fn tan_d(value: f64) -> f64 { f64::tan(value * deg2rad) }
pub fn asin_d(value: f64) -> f64 { f64::asin(value * deg2rad) }
pub fn acos_d(value: f64) -> f64 { f64::acos(value * deg2rad) }
pub fn atan_d(value: f64) -> f64 { f64::atan(value * deg2rad) }

pub fn atan2_d(x: f64, y: f64) -> f64 {
	f64::atan2(x * deg2rad, y * deg2rad) * rad2deg
}

pub fn wrapDeg(value: f64) -> f64 { value - f64::floor(value / 360.) * 360. }
pub fn wrapHours(value: f64) -> f64 { value - f64::floor(value / 24.) * 24. }


// pub fn cbrt(value:f64)->f64 {
// 	if (value > 0.0){
// 		return f64::exp(f64::log(value) / 3.0);
// 	}
// 	else if (x < 0.0){
// 		return -cbrt(-x);
// 	}
// 	/* x == 0.0 */
// 	return 0.0;
// }
pub fn cube_root(x: f64) -> f64 {
	if x == 0.0 {
		return 0.0;
	}
	let sign = x.signum();
	let abs_x = x.abs();
	let mut guess = abs_x;
	while (guess * guess * guess - abs_x).abs() > 1e-15 {
		guess = (2.0 * guess + abs_x / (guess * guess)) / 3.0;
	}
	sign * guess
}
// const earthTiltRad = earthTiltDeg * deg2rad
