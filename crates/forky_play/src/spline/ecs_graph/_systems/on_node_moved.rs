use crate::spline::ecs_graph::*;
use crate::spline::graph::*;
use bevy::prelude::*;




pub fn on_node_moved(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut graph_lookup: ResMut<EcsSplineGraphLookup>,
	mut query: Query<
		(&Transform, &EcsSplineGraphId, &SplineNode),
		Changed<Transform>,
	>,
) {
	for (transform, graph_id, node) in query.iter_mut() {
		let graph = graph_lookup.get_mut(&graph_id).unwrap();
		graph.update_node_position(
			&mut commands,
			&mut meshes,
			node,
			transform.translation,
		);
	}
}
