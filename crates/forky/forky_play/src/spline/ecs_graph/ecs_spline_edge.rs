use super::*;
use crate::materials::RenderBundle;
use crate::spline::graph::SplineEdge;
use crate::spline::graph::SplineNode;
use bevy::prelude::*;


#[derive(Debug, PartialEq, Clone)]
pub struct EcsSplineEdge {
	pub node1: SplineNode,
	pub node2: SplineNode,
	// pub edge: SplineEdge,
	pub mesh: Entity,
	pub handles: Vec<Entity>,
}

#[derive(Bundle)]
pub struct SplineEdgeBundle<T: Material> {
	pub transform: TransformBundle,
	pub render: RenderBundle<T>,
	pub edge: SplineEdge,
	pub graph_id: EcsSplineGraphId,
}

impl<T: Material> SplineEdgeBundle<T> {
	pub fn new(
		edge: SplineEdge,
		material: Handle<T>,
		graph_id: EcsSplineGraphId,
	) -> Self {
		Self {
			transform: TransformBundle::default(),
			render: RenderBundle::new(Handle::<Mesh>::default(), material),
			edge,
			graph_id,
		}
	}
}
