use super::*;

pub fn wrap(value: f64, wrap: f64) -> f64 {
	value - f64::floor(value / wrap) * wrap
}

//https://stackoverflow.com/questions/1878907/how-can-i-find-the-difference-between-two-angles
pub fn angle_between(a: f64, b: f64) -> f64 { wrap(a - b + PI, TAU) - PI }
pub fn unsigned_angle_between(a: f64, b: f64) -> f64 {
	f64::abs(angle_between(a, b))
}
pub fn angle_between_d(a: f64, b: f64) -> f64 {
	wrap(a - b + 180., 360.) - 180.
}
pub fn unsigned_angle_between_d(a: f64, b: f64) -> f64 {
	f64::abs(angle_between_d(a, b))
}
