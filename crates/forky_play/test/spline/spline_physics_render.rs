use bevy::ecs::query;
use bevy::prelude::*;
use forky_play::*;
use sweet::*;

sweet! {

	test skip "spline physics render" {
		let mut app = App::new();
		app.__()
			.add_systems(Update,utility::create_exit_after_system(3.))
			.add_plugins(plugins::ForkyDebugPlugin::default())
			.add_plugins(spline::graph::SplineGraphPlugin)
			.add_plugins(spline::physics::SplinePhysicsPlugin)
			// .add_systems(Startup, spawn_spline_cube)
			.add_systems(Startup, spline::utils::spawn_spline_graph_cube)
			.run();
		}
}
