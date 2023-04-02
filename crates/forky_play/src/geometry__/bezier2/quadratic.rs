#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::bezier;
use bevy::prelude::*;

pub fn quadratic(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
	Vec2::new(
		bezier::quadratic(p0.x, p1.x, p2.x, t),
		bezier::quadratic(p0.y, p1.y, p2.y, t),
	)
}
pub fn quadratic_derivative(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
	Vec2::new(
		bezier::quadratic_derivative(p0.x, p1.x, p2.x, t),
		bezier::quadratic_derivative(p0.y, p1.y, p2.y, t),
	)
}
pub fn quadratic_derivative2(p0: Vec2, p1: Vec2, p2: Vec2) -> Vec2 {
	Vec2::new(
		bezier::quadratic_derivative2(p0.x, p1.x, p2.x),
		bezier::quadratic_derivative2(p0.y, p1.y, p2.y),
	)
}
pub fn quadratic_derivative3(p0: Vec2, p1: Vec2, p2: Vec2) -> Vec2 {
	Vec2::new(
		bezier::quadratic_derivative3(p0.x, p1.x, p2.x),
		bezier::quadratic_derivative3(p0.y, p1.y, p2.y),
	)
}
