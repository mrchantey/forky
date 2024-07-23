#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::bezier;
use bevy::prelude::*;

pub fn linear(p0: Vec2, p1: Vec2, t: f32) -> Vec2 {
	Vec2::new(
		bezier::linear(p0.x, p1.x, t), 
		bezier::linear(p0.y, p1.y, t)
	)
}
pub fn linear_derivative(p0: Vec2, p1: Vec2) -> Vec2 {
	Vec2::new(
		bezier::linear_derivative(p0.x, p1.x), 
		bezier::linear_derivative(p0.y, p1.y)
	)
}
