use crate::bezier;
use bevy::prelude::*;

pub fn linear(p0: Vec3, p1: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::linear(p0.x, p1.x, t),
		bezier::linear(p0.y, p1.y, t),
		bezier::linear(p0.z, p1.z, t),
	)
}

pub fn linear_derivative(p0: Vec3, p1: Vec3) -> Vec3 {
	Vec3::new(
		bezier::linear_derivative(p0.x, p1.x),
		bezier::linear_derivative(p0.y, p1.y),
		bezier::linear_derivative(p0.z, p1.z),
	)
}
