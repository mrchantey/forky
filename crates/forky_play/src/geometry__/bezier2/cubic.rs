#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::bezier;
use bevy::prelude::*;

pub fn cubic(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
	Vec2::new(
		bezier::cubic(p0.x, p1.x, p2.x, p3.x, t),
		bezier::cubic(p0.y, p1.y, p2.y, p3.y, t),
	)
}
pub fn cubic_derivative(p0: Vec2,p1: Vec2,p2: Vec2,p3: Vec2,t: f32) -> Vec2 {
	Vec2::new(
		bezier::cubic_derivative(p0.x, p1.x, p2.x, p3.x, t),
		bezier::cubic_derivative(p0.y, p1.y, p2.y, p3.y, t),
	)
}
pub fn cubic_derivative2(p0: Vec2,p1: Vec2,p2: Vec2,p3: Vec2,t: f32,) -> Vec2 {
	Vec2::new(
		bezier::cubic_derivative2(p0.x, p1.x, p2.x, p3.x, t),
		bezier::cubic_derivative2(p0.y, p1.y, p2.y, p3.y, t),
	)
}
pub fn cubic_derivative3(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) -> Vec2 {
	Vec2::new(
		bezier::cubic_derivative3(p0.x, p1.x, p2.x, p3.x),
		bezier::cubic_derivative3(p0.y, p1.y, p2.y, p3.y),
	)
}