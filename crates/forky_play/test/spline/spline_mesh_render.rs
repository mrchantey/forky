use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;

use sweet::*;
sweet! {

	test skip "spline mesh" {
		let mut app = App::new();
		app.__()
		.forky_exit_after(10.)
			.add_plugin(plugins::ForkyDebugPlugin)
			.add_plugin(ecs_graph::EcsSplineGraphPlugin)
			.add_plugin(spline::physics::SplinePhysicsPlugin)
			.add_startup_system(spawn_spline)
			.run();
		}
}

fn spawn_spline(mut commands: Commands) {
	commands.spawn((Spline::Cubic(CubicSpline {
		p0: Vec3::new(-1., 1., 0.),
		p1: Vec3::new(-1., 0., 0.),
		p2: Vec3::new(1., 0., 0.),
		p3: Vec3::new(1., 1., 0.),
	}),));
}
