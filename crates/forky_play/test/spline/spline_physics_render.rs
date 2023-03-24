use bevy::ecs::query;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use forky_play::spline::*;
use forky_play::*;

use sweet::*;
sweet! {

	test "spline physics" {
		let mut app = App::new();
		app.__()
		.forky_exit_after(4.)
			.add_plugin(plugins::CustomDefaultPlugin)
			.add_plugin(spline::SplinePhysicsPlugin)
			.add_plugin(DebugLinesPlugin::with_depth_test(true))
			.add_startup_system(plugins::spawn_default_camera)
			.add_startup_system(spawn_spline_cube)
			.add_system(draw_spline)
			.run();
		}
}

fn spawn_spline_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
			material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			..default()
		},
		SplinePosition::default(),
		SplineVelocity::default(),
		physics::AccelerationForce(Vec3::DOWN),
		Spline::Cubic(CubicSpline {
			p0: Vec3::new(-1., 1., 0.),
			p1: Vec3::new(-1., 0., 0.),
			p2: Vec3::new(1., 0., 0.),
			p3: Vec3::new(1., 1., 0.),
		}),
	));
}



pub fn draw_spline(mut lines: ResMut<DebugLines>, query: Query<&Spline>) {
	let num_nodes = 10;

	for spline in &query {
		let path = spline.path(num_nodes);

		for i in 0..path.len() - 1 {
			let i = i as usize;
			lines.line_colored(
				path[i],
				path[i + 1],
				0.0,
				Color::YELLOW.with_a(0.8),
			);
		}
	}
}
