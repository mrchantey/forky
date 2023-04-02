use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;

fn main() {
	let mut app = App::new();
	app.__()
		// .forky_exit_after(10.)
		.add_plugin(plugins::ForkyFullPlugin)
		.add_plugin(spline::SplinePlugin)
		.add_startup_system(setup)
		.add_system(spline::utils::draw_ecs_graph)
		.run();
}
fn setup(
	mut commands: Commands,
	mut graph_lookup: ResMut<spline::graph::EcsSplineGraphLookup>,
	interaction_settings: Res<tool::InteractionSettings>,
	interaction_resources: Res<tool::InteractionResources>,
	mut materials: ResMut<Assets<materials::UvMaterial>>,
) {
	let material = materials.add(materials::UvMaterial::default());
	let (_id, graph) = graph_lookup.create_graph(material);
	// let node = graph.create_node();

	let spline = Spline::Cubic(CubicSpline::new(
		Vec3::new(-1.0, 0., 1.),
		Vec3::new(-1., 0., 0.),
		Vec3::new(1., 0., 0.),
		Vec3::new(1., 0., 1.),
	));

	graph.create_edge_from_spline(
		&mut commands,
		interaction_settings,
		interaction_resources,
		spline,
	);
}
