use crate::spline::ecs_graph::EcsSplineGraphId;
use crate::spline::ecs_graph::EcsSplineGraphLookup;
use crate::spline::graph::*;
use crate::spline::*;
use crate::tool::*;
use bevy::prelude::*;

pub fn link_spline_nodes(
	mut commands: Commands,
	keys: Res<ButtonInput<KeyCode>>,
	mut graphs: ResMut<EcsSplineGraphLookup>,
	selected_node_query: Query<
		(&Transform, &SplineNode, &EcsSplineGraphId),
		With<Selected>,
	>,
) {
	if !keys.just_pressed(KeyCode::KeyL) {
		return;
	}

	let mut combinations = selected_node_query.iter_combinations();
	while let Some(
		[(transform1, node1, graph_id1), (transform2, node2, graph_id2)],
	) = combinations.fetch_next()
	{
		if **graph_id1 != **graph_id2 {
			continue;
		}
		let graph = graphs.get_mut(&graph_id1.0).unwrap();
		if graph.graph.contains_edge(*node1, *node2) {
			continue;
		}
		let spline = Spline::Linear(LinearSpline::new(
			transform1.translation,
			transform2.translation,
		));

		graph.create_edge_with_spline(&mut commands, *node1, *node2, spline);

		// let new_node = graph.add_node(node.position);
		// graph.add_edge(node.id,new_node.id);
	}
}
