use crate::spline::ecs_graph::*;
use bevy::prelude::*;

//TODO this is overkill, should only update edges with changed nodes
pub fn apply_catmull_rom(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut graph_lookup: ResMut<EcsSplineGraphLookup>,
) {
	if graph_lookup.is_changed() {
		// println!("updating graph..");
		for graph in graph_lookup.values_mut() {
			graph.apply_catmull_rom(&mut commands, &mut meshes);
		}
	}
}
