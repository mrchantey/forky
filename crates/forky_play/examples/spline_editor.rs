use bevy::prelude::*;
use forky_play::*;

fn main() {
	let mut app = App::new();
	app.__()
		.add_plugins(plugins::ForkyFullPlugin::default())
		.add_plugins(spline::SplinePlugin)
		.add_systems(Startup, setup)
		.run();
}
fn setup(
	mut graph_lookup: ResMut<spline::ecs_graph::EcsSplineGraphLookup>,
	mut materials: ResMut<Assets<materials::UvMaterial>>,
) {
	let material = materials.add(materials::UvMaterial::default());
	graph_lookup.create_graph(material);
}
