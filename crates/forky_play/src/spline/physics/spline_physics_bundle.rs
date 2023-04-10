use super::*;
use crate::{
	physics::Friction,
	spline::{ecs_graph::EcsSplineGraphId, graph::SplineEdge},
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
	edge: SplineEdge,
	graph_id: EcsSplineGraphId,
}

impl SplinePhysicsBundle {
	pub fn new_default(
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		edge: SplineEdge,
		graph_id: EcsSplineGraphId,
	) -> Self {
		Self::new(
			meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
			materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
			edge,
			graph_id,
		)
	}
	pub fn new(
		mesh: Handle<Mesh>,
		material: Handle<StandardMaterial>,
		edge: SplineEdge,
		graph_id: EcsSplineGraphId,
	) -> Self {
		Self {
			pbr: PbrBundle {
				mesh,
				material,
				..default()
			},
			position: spline::physics::SplinePosition::default(),
			velocity: spline::physics::SplineVelocity::default(),
			friction: physics::Friction(0.1),
			acceleration: physics::AccelerationForce(Vec3::DOWN),
			edge,
			graph_id,
		}
	}
	pub fn with_velocity(mut self, velocity: f32) -> Self {
		*self.velocity = velocity;
		self
	}
	pub fn with_friction(mut self, friction: f32) -> Self {
		*self.friction = friction;
		self
	}
}
