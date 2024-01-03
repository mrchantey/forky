use super::*;
use crate::spline::graph::SplineEdge;
use crate::spline::SplineType;
use crate::*;
use bevy::prelude::*;

pub fn update_velocity_from_impulse(
	mut query_impulse: Query<(
		&mut physics::AccelerationImpulse,
		&mut SplineVelocity,
		&SplinePosition,
		&SplineEdge,
	)>,
) {
	for (mut acceleration, mut velocity, position, edge) in
		query_impulse.iter_mut()
	{
		**velocity += edge.spline.acceleration(**position, **acceleration);
		**acceleration = Vec3::ZERO;
	}
}
pub fn update_velocity_from_force(
	mut query: Query<(
		&physics::AccelerationForce,
		&mut SplineVelocity,
		&SplinePosition,
		&SplineEdge,
	)>,
	time: Res<Time>,
) {
	for (acceleration, mut velocity, position, edge) in query.iter_mut() {
		**velocity += edge
			.spline
			.acceleration(**position, **acceleration * time.delta_seconds());
	}
}

pub fn update_velocity_from_friction(
	mut query: Query<(&physics::Friction, &mut SplineVelocity)>,
	time: Res<Time>,
) {
	for (friction, mut velocity) in query.iter_mut() {
		let force = velocity.signum() * -1. * **friction * time.delta_seconds();
		**velocity += force;
	}
}

pub fn update_spline_position(
	mut query: Query<(&mut SplinePosition, &SplineVelocity)>,
	time: Res<Time>,
) {
	for (mut position, velocity) in query.iter_mut() {
		**position += **velocity * time.delta_seconds();
		// println!("pos: {}, velocity: {}", **position, **velocity);
	}
}
pub fn update_transform_position(
	mut query: Query<(&mut Transform, &SplinePosition, &SplineEdge)>,
) {
	for (mut transform, position, edge) in query.iter_mut() {
		transform.translation = edge.spline.position(**position);
	}
}
