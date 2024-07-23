use super::*;
use crate::spline::Spline;
use bevy::prelude::*;
use std::borrow::Borrow;

#[derive(
	Component, Debug, Deref, DerefMut, Clone, Copy, Eq, PartialEq, Hash,
)]
pub struct SplineEdgeId(pub u64);

impl Borrow<u64> for SplineEdgeId {
	fn borrow(&self) -> &u64 { &self.0 }
}

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct SplineEdge {
	pub id: SplineEdgeId,
	/// The start node where t == 0
	pub a: SplineNode,
	/// The end node where t == 1
	pub b: SplineNode,
	pub spline: Spline,
}
impl SplineEdge {
	pub fn new(
		id: SplineEdgeId,
		a: SplineNode,
		b: SplineNode,
		spline: Spline,
	) -> Self {
		Self { id, a, b, spline }
	}
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SplineLink {
	pub a: SplineNode,
	pub b: SplineNode,
}
impl SplineLink {
	pub fn new(a: SplineNode, b: SplineNode) -> Self { Self { a, b } }
}
