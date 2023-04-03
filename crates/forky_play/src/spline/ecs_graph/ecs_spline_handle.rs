use super::*;
use crate::spline::graph::SplineEdgeId;
use crate::tool;
use bevy::prelude::*;


#[derive(Component, Deref, DerefMut, Debug, Copy, Clone, Eq, PartialEq)]
pub struct SplineHandleIndex(pub u32);

#[derive(Bundle, Clone)]
pub struct EcsSplineHandleBundle {
	pub transform: TransformBundle,
	pub handle_index: SplineHandleIndex,
	pub edge_id: SplineEdgeId,
	pub graph_id: EcsSplineGraphId,
	pub interactable: tool::Interactable,
}

impl EcsSplineHandleBundle {
	pub fn new(
		position: Vec3,
		handle_index: SplineHandleIndex,
		edge_id: SplineEdgeId,
		graph_id: EcsSplineGraphId,
	) -> Self {
		Self {
			transform: TransformBundle::from(Transform::from_translation(
				position,
			)),
			handle_index,
			edge_id,
			graph_id,
			interactable: tool::Interactable,
		}
	}
}
