use super::*;
use crate::spline::Spline;
use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct SplineEdge {
	/// The start node where t == 0
	pub a: SplineNode,
	/// The end node where t == 1
	pub b: SplineNode,
	pub spline: Spline,
}
impl SplineEdge {
	pub fn new(a: SplineNode, b: SplineNode, spline: Spline) -> Self {
		Self { a, b, spline }
	}
}
