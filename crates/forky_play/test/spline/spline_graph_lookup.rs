use bevy::prelude::*;
use forky_play::{spline::graph::*, spline::*, *};
use sweet::*;

sweet! {
	it "works" {
		let mut app = App::new();

		app.__()
			.add_plugin(SplinePlugin)
			// .add_startup_system(setup)
			.__();


		let mut graph_lookup = app.world.resource_mut::<SplineGraphLookup>();
		let (graph_id,graph) = graph_lookup.create_graph();

		let node = graph.create_node();

		let entity = app.world.spawn(spline::graph::SplineNodeBundle::new(
			Vec3::ZERO,
			node,
			graph_id,
		)).id();


		app.update();
	}
}


// fn setup(mut commands: Commands, graph_lookup: ResMut<SplineGraphLookup>) {
// 	commands.spawn()
// }
