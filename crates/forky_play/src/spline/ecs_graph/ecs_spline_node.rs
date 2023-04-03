use super::*;
use crate::spline::graph::SplineNode;
use crate::tool;
use bevy::prelude::*;


#[derive(
	Component, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
pub struct EcsSplineNode {
	id: u64,
	entity: Entity,
}


#[derive(Bundle, Clone)]
pub struct EcsSplineNodeBundle {
	pub transform: TransformBundle,
	pub node: SplineNode,
	pub graph_id: EcsSplineGraphId,
	pub interactable: tool::Interactable,
}

impl EcsSplineNodeBundle {
	pub fn new(
		position: Vec3,
		node: SplineNode,
		graph_id: EcsSplineGraphId,
	) -> Self {
		Self {
			transform: TransformBundle::from(Transform::from_translation(
				position,
			)),
			node,
			graph_id,
			interactable: tool::Interactable,
		}
	}
}
