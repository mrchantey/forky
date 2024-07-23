use super::*;
use crate::*;
use bevy::prelude::*;

pub fn spawn_initial_graph(
	mut commands: Commands,
	mut graph_lookup: ResMut<spline::ecs_graph::EcsSplineGraphLookup>,
	mut materials: ResMut<Assets<materials::UvMaterial>>,
) {
	let height: f32 = 3.;
	let width: f32 = 3.;

	let material = materials.add(materials::UvMaterial::default());
	let graph = graph_lookup.create_graph(material);

	let node1 = graph
		.create_node(&mut commands, Vec3::new(-width, height, 0.))
		.node;
	let node2 = graph
		.create_node(&mut commands, Vec3::new(0., height - 5., 0.))
		.node;
	let node3 = graph
		.create_node(&mut commands, Vec3::new(width, height, 0.))
		.node;

	graph.create_edge(&mut commands, node1, node2);
	graph.create_edge(&mut commands, node2, node3);

	// let edge = graph.create_edge_from_spline(&mut commands, spline1);

	commands.insert_resource(GraphSettings {
		start_node: node1,
		end_node: node3,
		graph_id: graph.id,
	})
}
