use bevy::prelude::*;
use forky_play::*;
use sweet::*;
sweet! {

	test skip "spline mesh" {
		let mut app = App::new();
		app.__()
		.forky_exit_after(10.)
			.add_plugins(plugins::ForkyDebugPlugin::default())
			.add_plugins(spline::ecs_graph::EcsSplineGraphPlugin)
			.add_plugins(spline::physics::SplinePhysicsPlugin)
			.add_systems(Startup, spawn_spline)
			.run();
		}
}

fn spawn_spline(mut commands: Commands) {
	commands.spawn((spline::Spline::Cubic(spline::CubicSpline {
		p0: Vec3::new(-1., 1., 0.),
		p1: Vec3::new(-1., 0., 0.),
		p2: Vec3::new(1., 0., 0.),
		p3: Vec3::new(1., 1., 0.),
	}),));
}
