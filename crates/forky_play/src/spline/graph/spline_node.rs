use bevy::prelude::*;

use derive_deref::{Deref, DerefMut};

use super::*;

#[derive(
	Component,
	Deref,
	DerefMut,
	Debug,
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Hash,
)]
pub struct SplineNode(pub u64);

#[derive(Bundle)]
pub struct SplineNodeBundle {
	pub node: SplineNode,
	pub transform: TransformBundle,
}

impl SplineNodeBundle {
	pub fn new(position: Vec3, node: SplineNode) -> Self {
		SplineNodeBundle {
			transform: TransformBundle::from(Transform::from_translation(
				position,
			)),
			node,
		}
	}
}

// #[derive(Component, Debug, Default, Clone, PartialEq)]
// pub struct SplineNodeHandle {
// 	pub edges: Vec<Entity>,
// }


pub fn on_node_moved(
	mut graph_lookup: ResMut<SplineGraphLookup>,
	query: Query<(&Transform, &SplineNode, &SplineGraphId), Changed<Transform>>,
) {
	for (transform, node, graph_id) in query.iter() {
		let _graph = graph_lookup.get_mut(&graph_id).unwrap();

		// graph.get_
		println!("Node {} moved to {:?}", node.0, transform.translation);
	}
}
