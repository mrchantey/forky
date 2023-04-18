use std::f64::consts::{PI, TAU};



pub const DEG2RAD: f64 = PI / 180.;
pub const RAD2DEG: f64 = 180. / PI;
pub const RAD2HOURS: f64 = 12. / PI;
pub const HOURS2RAD: f64 = PI / 12.;
pub const DEG2HOURS: f64 = 1. / 15.;
pub const HOURS2DEG: f64 = 15.;

// pub const earthTiltDeg:f64 = 23.4393;
pub const EARTH_TILT_DEG: f64 = 23.43656;
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

pub fn wrap_rad(value: f64) -> f64 { wrap(value, TAU) }
pub fn wrap_deg(value: f64) -> f64 { wrap(value, 360.) }
pub fn wrap_hours(value: f64) -> f64 { wrap(value, 24.) }

pub fn wrap(value: f64, wrap: f64) -> f64 {
	value - f64::floor(value / wrap) * wrap
}
// const earthTiltRad = earthTiltDeg * deg2rad
