use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;
use sweet::*;

sweet! {
	it skip "works" {
		let mut app = App::new();

		app.__()
			.add_plugins(plugins::CustomDefaultPlugin::default())
			.add_plugins(materials::ForkyMaterialPlugin)
			.add_plugins(ecs_graph::EcsSplineGraphPlugin)
			.add_systems(Startup, setup)
			.__();

		// let (graph_id,graph) = graph_lookup.create_graph();

		// app.update();
	}
}


fn setup(
	mut commands: Commands,
	mut graph_lookup: ResMut<ecs_graph::EcsSplineGraphLookup>,
	mut materials: ResMut<Assets<materials::UvMaterial>>,
) {
	let material = materials.add(materials::UvMaterial::default());
	let graph = graph_lookup.create_graph(material);
}
