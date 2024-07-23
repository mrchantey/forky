#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::bezier;
use bevy::prelude::*;


pub fn cubic(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::cubic(p0.x, p1.x, p2.x, p3.x, t),
		bezier::cubic(p0.y, p1.y, p2.y, p3.y, t),
		bezier::cubic(p0.z, p1.z, p2.z, p3.z, t),
	)
}

pub fn cubic_derivative(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::cubic_derivative(p0.x, p1.x, p2.x, p3.x, t),
		bezier::cubic_derivative(p0.y, p1.y, p2.y, p3.y, t),
		bezier::cubic_derivative(p0.z, p1.z, p2.z, p3.z, t),
	)
}
pub fn cubic_derivative2(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::cubic_derivative2(p0.x, p1.x, p2.x, p3.x, t),
		bezier::cubic_derivative2(p0.y, p1.y, p2.y, p3.y, t),
		bezier::cubic_derivative2(p0.z, p1.z, p2.z, p3.z, t),
	)
}
pub fn cubic_derivative3(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Vec3 {
	Vec3::new(
		bezier::cubic_derivative3(p0.x, p1.x, p2.x, p3.x),
		bezier::cubic_derivative3(p0.y, p1.y, p2.y, p3.y),
		bezier::cubic_derivative3(p0.z, p1.z, p2.z, p3.z),
	)
}
