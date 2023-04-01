use super::*;
use crate::spline::Spline;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Debug, Default, Clone, PartialEq)]
pub struct SplineEdgeList(pub Vec<u64>);

#[derive(Debug, PartialEq, Clone)]
pub struct EcsSplineEdge {
	pub link: SplineLink,
	pub mesh: Entity,
	pub points: Vec<Entity>,
}

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct SplineEdge {
	// pub id: u64,
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

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct SplineLink {
	pub a: SplineNode,
	pub b: SplineNode,
}
impl SplineLink {
	pub fn new(a: SplineNode, b: SplineNode) -> Self { Self { a, b } }
}
