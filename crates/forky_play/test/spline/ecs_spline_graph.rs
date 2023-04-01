use bevy::prelude::*;
use forky_play::{spline::graph::*, spline::*, *};
use sweet::*;

sweet! {
	it skip "works" {
		let mut app = App::new();

		app.__()
			.add_plugin(plugins::CustomDefaultPlugin)
			.add_plugin(materials::ForkyMaterialPlugin)
			.add_plugin(SplinePlugin)
			.add_startup_system(setup)
			.__();

		// let (graph_id,graph) = graph_lookup.create_graph();

		// app.update();
	}
}


fn setup(
	mut commands: Commands,
	mut graph_lookup: ResMut<EcsSplineGraphLookup>,
	mut materials: ResMut<Assets<materials::UvMaterial>>,
) {
	let material = materials.add(materials::UvMaterial::default());
	let (_id, graph) = graph_lookup.create_graph(material);
	// let node = graph.create_node();
}
