use crate::spline::ecs_graph::EcsSplineGraphId;
use crate::spline::ecs_graph::EcsSplineGraphLookup;
use crate::spline::graph::*;
use crate::spline::*;
use crate::tool::*;
use bevy::prelude::*;



pub fn create_spline_node(
	mut commands: Commands,
	keys: Res<Input<KeyCode>>,
	camera_ray: Res<CameraRay>,
	mut graphs: ResMut<EcsSplineGraphLookup>,
	selected_node_query: Query<
		(Entity, &Transform, &SplineNode, &EcsSplineGraphId),
		With<Selected>,
	>,
) {
	if !keys.just_pressed(KeyCode::N) {
		return;
	}

	let intersect = match &camera_ray.ground_intersect {
		Some(intersect) => intersect,
		None => return,
	};

	//hack
	let graph = graphs.values_mut().next().unwrap();
	let (entity, node1) = graph.create_node(&mut commands, intersect.position);

	commands
		.entity(entity)
		.insert((Selected, PrimaryInteracted));

	//TODO share code with link_spline_nodes
	for (entity, transform, node2, graph_id) in selected_node_query.iter() {
		commands
			.entity(entity)
			.remove::<(Selected, PrimaryInteracted)>();

		let graph = graphs.get_mut(&graph_id).unwrap();

		if **graph_id != *graph.id {
			continue;
		}

		if graph.graph.contains_edge(node1, *node2) {
			continue;
		}
		let spline = Spline::Linear(LinearSpline::new(
			intersect.position,
			transform.translation,
		));
		graph.create_edge(&mut commands, node1, *node2, spline);
	}
}
