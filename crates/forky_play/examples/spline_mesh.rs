use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use forky_play::spline::*;
use forky_play::*;

fn main() {
	let mut app = App::new();
	app.__()
		// .forky_exit_after(10.)
		.add_plugin(plugins::ForkyFullPlugin)
		.add_plugin(spline::tool::SplineToolPlugin)
		.add_plugin(spline::physics::SplinePhysicsPlugin)
		.add_startup_system(spawn_spline)
		.add_system(spline::mesh::append_spline_mesh)
		.run();
}

fn spawn_spline(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let p0 = Vec3::new(-1., 1., 0.);
	let p3 = Vec3::new(1., 1., 0.);

	let mesh = meshes.add(Mesh::from(shape::UVSphere {
		radius: 0.1,
		sectors: 8,
		stacks: 8,
	}));

	commands.spawn(spline::graph::SplineNodeBundle::new(
		p0, &mesh, &mut materials, 0.1,
	));
	commands.spawn(spline::graph::SplineNodeBundle::new(
		p3, &mesh, &mut materials, 0.1,
	));

	commands.spawn(
		(Spline::Cubic(CubicSpline {
			p0,
			p1: Vec3::new(-1., 0., 0.),
			p2: Vec3::new(1., 0., 0.),
			p3,
		})),
	);
}
