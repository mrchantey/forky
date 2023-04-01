use super::*;
use crate::spline::Spline;
use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct SplineEdge {
	pub id: u64,
	/// The start node where t == 0
	pub a: SplineNode,
	/// The end node where t == 1
	pub b: SplineNode,
	pub spline: Spline,
}
impl SplineEdge {
	pub fn new(id: u64, a: SplineNode, b: SplineNode, spline: Spline) -> Self {
		Self { id, a, b, spline }
	}
}
