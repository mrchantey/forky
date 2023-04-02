use crate::spline::graph::*;
use bevy::prelude::*;
// pub fn on_ecs_node_moved(
// 	mut commands: Commands,
// 	mut meshes: ResMut<Assets<Mesh>>,
// 	mut graph_lookup: ResMut<EcsSplineGraphLookup>,
// 	mut query: Query<
// 		(
// 			&Transform,
// 			&SplinePointIndex,
// 			&EcsSplineGraphId,
// 			&SplineNode,
// 		),
// 		Changed<Transform>,
// 	>,
// ) {
// 	for (transform, point, graph_id, node) in query.iter_mut() {
// 		let graph = graph_lookup.get_mut(&graph_id).unwrap();
// 		graph.update_edge_from_node(node, transform.translation);

// 		for (entity, edge) in
// 			edges.iter().map(|e| (graph.edges.get(&e.id).unwrap(), e))
// 		{
// 		}
// 	}
// }


pub fn on_ecs_point_moved(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut graph_lookup: ResMut<EcsSplineGraphLookup>,
	mut query: Query<
		(
			&Transform,
			&SplinePointIndex,
			&EcsSplineGraphId,
			&SplineEdgeList,
		),
		Changed<Transform>,
	>,
) {
	for (transform, point, graph_id, edges) in query.iter_mut() {
		let graph = graph_lookup.get_mut(&graph_id).unwrap();

		graph.update_edge_from_point(
			&mut commands,
			&mut meshes,
			*point,
			transform.translation,
			edges,
		);
	}
}
