use bevy::ecs::query;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use forky_play::spline::*;
use forky_play::*;

use sweet::*;

sweet! {

	test skip "spline physics render" {
		let mut app = App::new();
		app.__()
		.forky_exit_after(10.)
			.add_plugin(plugins::ForkyDebugPlugin::default())
			.add_plugin(spline::graph::SplineGraphPlugin)
			.add_plugin(spline::physics::SplinePhysicsPlugin)
			// .add_startup_system(spawn_spline_cube)
			.add_startup_system(spline::utils::spawn_spline_graph_cube)
			.run();
		}
}
