use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;

fn main() {
	let mut app = App::new();
	app.__()
		// .forky_exit_after(10.)
		.add_plugin(plugins::ForkyFullPlugin)
		.add_plugin(spline::SplineEcsGraphPlugin)
		.add_plugin(spline::tool::SplineToolPlugin)
		.add_plugin(spline::physics::SplinePhysicsPlugin)
		.add_startup_system(setup)
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

	let spline = Spline::Cubic(CubicSpline {
		p0: Vec3::new(-1.0, 1., 0.),
		p1: Vec3::new(-1., 0., 0.),
		p2: Vec3::new(1., 0., 0.),
		p3: Vec3::new(1., 1., 0.),
	});

	graph.create_edge_from_spline(
		&mut commands,
		interaction_settings,
		interaction_resources,
		spline,
	);
}
