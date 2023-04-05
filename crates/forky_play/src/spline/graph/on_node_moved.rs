use super::*;
use bevy::prelude::*;



pub fn on_node_moved(
	mut graph_lookup: ResMut<SplineGraphLookup>,
	query: Query<(&Transform, &SplineNode, &SplineGraphId), Changed<Transform>>,
) {
	for (transform, node, graph_id) in query.iter() {
		let _graph = graph_lookup.get_mut(&graph_id).unwrap();

		// graph.get_
		println!("Node {} moved to {:?}", **node, transform.translation);
	}
}
