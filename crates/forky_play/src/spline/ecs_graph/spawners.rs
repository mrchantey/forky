use crate::*;
use bevy::prelude::*;


pub fn create_graph_with_spline(
	spline: spline::Spline,
	create_handles: bool,
) -> impl Fn(
	Commands,
	ResMut<spline::ecs_graph::EcsSplineGraphLookup>,
	ResMut<Assets<materials::UvMaterial>>,
) {
	move |mut commands, mut graph_lookup, mut materials| {
		let material = materials.add(materials::UvMaterial::default());
		let graph =
			graph_lookup.create_graph_with_options(material, create_handles);

		graph.create_edge_from_spline(&mut commands, spline);
	}
}
