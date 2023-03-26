use bevy::ecs::query;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use forky_play::spline::*;
use forky_play::*;
use forky_play::physics;
use forky_play::spline::graph::SplineGraph;
// use forky_play::spline::physics;

pub fn spawn_spline_cube(
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
		spline::physics::SplinePosition::default(),
		spline::physics::SplineVelocity::default(),
		physics::Friction(0.1),
		physics::AccelerationForce(Vec3::DOWN),
		Spline::Cubic(CubicSpline {
			p0: Vec3::new(-1., 1., 0.),
			p1: Vec3::new(-1., 0., 0.),
			p2: Vec3::new(1., 0., 0.),
			p3: Vec3::new(1., 1., 0.),
		}),
	));
}

pub fn spawn_spline_graph_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let mut graph = SplineGraph::new();
	let spline1 = Spline::Quadratic(QuadraticSpline {
		p0: Vec3::new(-1., 1., 0.),
		p1: Vec3::new(-1., 0., 0.),
		p2: Vec3::new(0., 0., 0.),
	});
	let spline2 = Spline::Quadratic(QuadraticSpline {
		p0: Vec3::new(0., 0., 0.),
		p1: Vec3::new(1., 0., 0.),
		p2: Vec3::new(1., -1., 0.),
	});

	let node1 = graph.create_node();
	let node2 = graph.create_node();
	let node3 = graph.create_node();
	let edge12 = graph.create_edge(node1, node2, spline1);
	let edge23 = graph.create_edge(node2, node3, spline2);

	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
			material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			..default()
		},
		spline::physics::SplinePosition::default(),
		spline::physics::SplineVelocity::default(),
		physics::Friction(0.1),
		physics::AccelerationForce(Vec3::DOWN),
		edge12,
		spline1,
		graph,
	));
}

pub fn draw_spline(
	mut lines: ResMut<DebugLines>,
	query: Query<&Spline, Without<SplineGraph>>,
) {
	for spline in &query {
		draw(&mut lines, spline);
	}
}

pub fn draw_graph(mut lines: ResMut<DebugLines>, query: Query<&SplineGraph>) {
	let num_nodes = 10;

	for graph in &query {
		for edge in graph.all_edges() {
			draw(&mut lines, &edge.2.spline);
		}
	}
}
fn draw(lines: &mut DebugLines, spline: &Spline) {
	let num_nodes = 10;
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
