use crate::spline::ecs_graph::*;
use crate::spline::graph::*;
use bevy::prelude::*;


pub fn on_handle_moved(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut graph_lookup: ResMut<EcsSplineGraphLookup>,
	mut query: Query<
		(
			&Transform,
			&SplineHandleIndex,
			&SplineEdgeId,
			&EcsSplineGraphId,
		),
		Changed<Transform>,
	>,
) {
	for (transform, handle_index, edge_id, graph_id) in query.iter_mut() {
		let graph = graph_lookup.get_mut(&graph_id.0).unwrap();
		graph.update_edge_from_handle(
			&mut commands,
			&mut meshes,
			edge_id,
			transform.translation,
			handle_index,
		);
	}
}
