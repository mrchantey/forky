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

#[derive(Component, Deref, DerefMut, Debug, Copy, Clone, Eq, PartialEq)]
pub struct SplinePointIndex(pub u32);

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
