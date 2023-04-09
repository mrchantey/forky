use super::*;
use crate::{
	spline::{ecs_graph::EcsSplineGraphId, graph::SplineEdgeId},
	*,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct SplinePhysicsBundle {
	pbr: PbrBundle,
	position: SplinePosition,
	velocity: SplineVelocity,
	friction: physics::Friction,
	acceleration: physics::AccelerationForce,
	edge_id: SplineEdgeId,
	graph_id: EcsSplineGraphId,
}

impl SplinePhysicsBundle {
	pub fn new(
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		edge_id: SplineEdgeId,
		graph_id: EcsSplineGraphId,
	) -> Self {
		Self {
			pbr: PbrBundle {
				mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
				material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
				transform: Transform::from_xyz(0.0, 0.0, 0.0),
				..default()
			},
			position: spline::physics::SplinePosition::default(),
			velocity: spline::physics::SplineVelocity::default(),
			friction: physics::Friction(0.1),
			acceleration: physics::AccelerationForce(Vec3::DOWN),
			edge_id,
			graph_id,
		}
	}
}
