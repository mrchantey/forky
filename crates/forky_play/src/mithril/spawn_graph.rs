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

	let spline = spline::Spline::Cubic(spline::CubicSpline::new(
		Vec3::new(-width, height, 0.),
		Vec3::new(-width + 1., height, 0.),
		Vec3::new(width - 1., height - 5., 0.),
		Vec3::new(width, height - 5., 0.),
	));
	let edge = graph.create_edge_from_spline(&mut commands, spline);

	commands.insert_resource(GraphSettings {
		start_node: edge.node1,
		end_node: edge.node2,
		graph_id: graph.id,
	})
}
