use bevy::prelude::*;
use forky_play::*;

fn main() {
	let mut app = App::new();
	app.__()
		.add_plugin(plugins::ForkyFullPlugin::default())
		.add_plugin(spline::SplinePlugin)
		.add_startup_system(setup)
		.run();
}
fn setup(
	mut graph_lookup: ResMut<spline::ecs_graph::EcsSplineGraphLookup>,
	mut materials: ResMut<Assets<materials::UvMaterial>>,
) {
	let material = materials.add(materials::UvMaterial::default());
	graph_lookup.create_graph(material);
}
