use crate::physics;
use crate::spline::graph::*;
// use crate::spline::*;
use crate::*;
use bevy::prelude::*;

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
		spline::Spline::Cubic(spline::CubicSpline {
			p0: Vec3::new(-1., 1., 0.),
			p1: Vec3::new(-1., 0., 0.),
			p2: Vec3::new(1., 0., 0.),
			p3: Vec3::new(1., 1., 0.),
		}),
	));
}

pub fn spawn_spline_graph_cube(
	mut commands: Commands,
	mut graphs: ResMut<SplineGraphLookup>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let (graph_id, graph) = graphs.create_graph();
	let pos1 = Vec3::new(-1., 1., 0.);
	let pos2 = Vec3::new(0., 0., 0.);
	let pos3 = Vec3::new(1., -1., 0.);

	let spline1 = spline::Spline::Quadratic(spline::QuadraticSpline {
		p0: pos1,
		p1: Vec3::new(-1., 0., 0.),
		p2: pos2,
	});
	let spline2 = spline::Spline::Quadratic(spline::QuadraticSpline {
		p0: pos2,
		p1: Vec3::new(1., 0., 0.),
		p2: pos3,
	});

	let node1 = graph.create_node(pos1);
	let node2 = graph.create_node(pos2);
	let node3 = graph.create_node(pos3);
	let edge12 = graph.create_edge(node1, node2, spline1);
	let _edge23 = graph.create_edge(node2, node3, spline2);

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
		graph_id,
	));
}
