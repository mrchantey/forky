use crate::spline::graph::*;
use crate::spline::tool::*;
use bevy::prelude::*;



pub fn create_interactable(
	mut commands: Commands,
	keys: Res<Input<KeyCode>>,
	camera_ray: Res<CameraRay>,
	mut graphs: ResMut<EcsSplineGraphLookup>,
	selected_node_query: Query<
		(&SplineNode, &EcsSplineGraphId),
		With<PrimaryInteracted>,
	>,
) {
	if !keys.just_pressed(KeyCode::N) {
		return;
	}

	let intersect = match &camera_ray.ground_intersect {
		Some(intersect) => intersect,
		None => return,
	};
	let _entity = commands
		.spawn((
			TransformBundle::from(Transform::from_translation(
				intersect.position,
			)),
			Interactable,
		))
		.id();

	if let Ok((_node, graph_id)) = selected_node_query.get_single() {
		let _graph = graphs.get_mut(&graph_id).unwrap();

		//TODO auto conenct to selected nodes

		// let new_node = graph.add_node(node.position);
		// graph.add_edge(node.id,new_node.id);
	}
}
