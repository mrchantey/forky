use crate::spline::ecs_graph::EcsSplineGraphId;
use crate::spline::ecs_graph::EcsSplineGraphLookup;
use crate::spline::graph::*;
use crate::spline::*;
use crate::tool::*;
use bevy::prelude::*;



pub fn create_spline_node(
	mut commands: Commands,
	keys: Res<ButtonInput<KeyCode>>,
	camera_ray: Res<CameraRay>,
	mut graphs: ResMut<EcsSplineGraphLookup>,
	selected_node_query: Query<
		(Entity, &Transform, &SplineNode, &EcsSplineGraphId),
		With<Selected>,
	>,
) {
	if !keys.just_pressed(KeyCode::KeyN) {
		return;
	}
	let intersect = match &camera_ray.origin_intersect {
		Some(intersect) => intersect,
		None => return,
	};

	//TODO find common graph of selected handles?
	let graph = graphs.values_mut().next().unwrap();
	let node2 = graph.create_node(&mut commands, intersect.position);

	commands
		.entity(node2.entity)
		.insert((Selected, PrimaryInteracted));

	//TODO share code with link_spline_nodes
	for (entity, transform, node1, graph_id) in selected_node_query.iter() {
		commands
			.entity(entity)
			.remove::<(Selected, PrimaryInteracted)>();

		let graph = graphs.get_mut(&graph_id.0).unwrap();

		if **graph_id != *graph.id {
			continue;
		}

		if graph.graph.contains_edge(node2.node, *node1) {
			continue;
		}
		let spline = Spline::Linear(LinearSpline::new(
			intersect.position,
			transform.translation,
		));
		graph.create_edge_with_spline(
			&mut commands,
			*node1,
			node2.node,
			spline,
		);
	}
}
