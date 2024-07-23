use bevy::prelude::*;
use forky_play::*;

fn main() {
	let mut app = App::new();
	app.__()
		.add_systems(Update, utility::create_exit_after_system(10.))
		.add_plugins(plugins::ForkyFullPlugin::default())
		.add_plugins(spline::SplinePlugin)
		.add_systems(Startup, setup)
		.run();
}



fn setup(
	mut commands: Commands,
	mut graph_lookup: ResMut<spline::ecs_graph::EcsSplineGraphLookup>,
	mut materials: ResMut<Assets<materials::UvMaterial>>,
) {
	let material = materials.add(materials::UvMaterial::default());
	let graph = graph_lookup.create_graph(material);

	let node1 = graph.create_node(&mut commands, Vec3::new(-1., 1., 0.));
	let node2 = graph.create_node(&mut commands, Vec3::new(-1., -1., 0.));
	let node3 = graph.create_node(&mut commands, Vec3::new(1., -1., 0.));
	let node4 = graph.create_node(&mut commands, Vec3::new(1., 1., 0.));

	graph.create_edge(&mut commands, node1.node, node2.node);
	graph.create_edge(&mut commands, node2.node, node3.node);
	graph.create_edge(&mut commands, node3.node, node4.node);
	graph.create_edge(&mut commands, node4.node, node1.node);
}
