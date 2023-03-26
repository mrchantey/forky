use bevy::ecs::query;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use forky_play::spline::*;
use forky_play::*;

use forky_play::utility::spawn_camera_at_position;
use sweet::*;
sweet! {

	test "spline mesh" {
		let mut app = App::new();
		app.__()
		.forky_exit_after(10.)
			.add_plugin(plugins::CustomDefaultPlugin)
			.add_plugin(spline::physics::SplinePhysicsPlugin)
			.add_startup_system(spawn_camera_at_position(Vec3::new(5., 5., 5.)))
			.add_startup_system(plugins::spawn_default_lights)
			.add_startup_system(spawn_spline)
			.add_system(spline::mesh::append_spline_mesh)
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
