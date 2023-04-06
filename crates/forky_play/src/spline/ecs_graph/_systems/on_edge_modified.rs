use crate::spline::ecs_graph::*;
use crate::spline::graph::*;
use bevy::prelude::*;




pub fn on_edge_modified(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut graph_lookup: ResMut<EcsSplineGraphLookup>,
	mut query: Query<(&EcsSplineGraphId, &SplineEdge), Changed<SplineEdge>>,
) {
	for (graph_id, edge) in query.iter_mut() {
		let graph = graph_lookup.get_mut(graph_id).unwrap();
		graph.update_edge_mesh(&mut commands, &edge.id, &mut meshes);
	}
}
