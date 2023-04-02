#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::bezier;
use bevy::prelude::*;

pub fn quadratic(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::quadratic(p0.x, p1.x, p2.x, t),
		bezier::quadratic(p0.y, p1.y, p2.y, t),
		bezier::quadratic(p0.z, p1.z, p2.z, t),
	)
}
pub fn quadratic_derivative(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::quadratic_derivative(p0.x, p1.x, p2.x, t),
		bezier::quadratic_derivative(p0.y, p1.y, p2.y, t),
		bezier::quadratic_derivative(p0.z, p1.z, p2.z, t),
	)
}
pub fn quadratic_derivative2(p0: Vec3, p1: Vec3, p2: Vec3) -> Vec3 {
	Vec3::new(
		bezier::quadratic_derivative2(p0.x, p1.x, p2.x),
		bezier::quadratic_derivative2(p0.y, p1.y, p2.y),
		bezier::quadratic_derivative2(p0.z, p1.z, p2.z),
	)
}
pub fn quadratic_derivative3(p0: Vec3, p1: Vec3, p2: Vec3) -> Vec3 {
	Vec3::new(
		bezier::quadratic_derivative3(p0.x, p1.x, p2.x),
		bezier::quadratic_derivative3(p0.y, p1.y, p2.y),
		bezier::quadratic_derivative3(p0.z, p1.z, p2.z),
	)
}
