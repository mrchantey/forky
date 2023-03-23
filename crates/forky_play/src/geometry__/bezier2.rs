use super::*;
use bevy::prelude::*;


#[rustfmt::skip]
pub fn linear(p0: Vec2, p1: Vec2, t: f32) -> Vec2 {
	Vec2::new(
		bezier::linear(p0.x, p1.x, t), 
		bezier::linear(p0.y, p1.y, t)
	)
}

pub fn quadratic(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
	Vec2::new(
		bezier::quadratic(p0.x, p1.x, p2.x, t),
		bezier::quadratic(p0.y, p1.y, p2.y, t),
	)
}

pub fn cubic(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
	Vec2::new(
		bezier::cubic(p0.x, p1.x, p2.x, p3.x, t),
		bezier::cubic(p0.y, p1.y, p2.y, p3.y, t),
	)
}
