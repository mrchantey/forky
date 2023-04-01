use crate::spline::mesh;
use bevy::prelude::*;

// use crate::*;
use super::*;


pub fn on_ecs_node_moved(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut graph_lookup: ResMut<EcsSplineGraphLookup>,
	query: Query<
		(&Transform, &SplineNode, &EcsSplineGraphId),
		Changed<Transform>,
	>,
) {
	for (transform, node, graph_id) in query.iter() {
		let graph = graph_lookup.get_mut(&graph_id).unwrap();
		graph.update_node_position(node, transform.translation);

		for (entity, edge) in graph.edge_entities(node) {
			let mesh = meshes.add(mesh::create_spline_mesh(
				&edge.spline,
				graph.edge_subdivisions,
			));

			commands.entity(*entity).insert(mesh);

			// println!("edge{:?}", edge);
		}
	}
}
