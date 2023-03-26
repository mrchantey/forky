use super::*;
use crate::spline::graph::SplineEdge;
use crate::spline::graph::SplineGraph;
use crate::spline::Spline;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::na::ComplexField;
use derive_deref::{Deref, DerefMut};

pub struct SplinePhysicsPlugin;

#[rustfmt::skip]
impl Plugin for SplinePhysicsPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_systems((
				update_velocity_from_impulse,
				update_velocity_from_force,
				update_velocity_from_friction,
				update_spline_position,
				update_current_spline,
				update_transform_position,
				)
				.in_set(physics::EulerPhysicsSet::Update)
				.chain()
			)
			.__();
	}
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct SplineVelocity(pub f32);

#[derive(Default, Component, Deref, DerefMut)]
pub struct SplinePosition(pub f32);




pub fn update_velocity_from_impulse(
	mut query_impulse: Query<(
		&mut physics::AccelerationImpulse,
		&mut SplineVelocity,
		&SplinePosition,
		&Spline,
	)>,
) {
	for (mut acceleration, mut velocity, position, spline) in
		query_impulse.iter_mut()
	{
		**velocity += spline.acceleration(**position, **acceleration);
		**acceleration = Vec3::ZERO;
	}
}
pub fn update_velocity_from_force(
	mut query: Query<(
		&physics::AccelerationForce,
		&mut SplineVelocity,
		&SplinePosition,
		&Spline,
	)>,
	time: Res<Time>,
) {
	for (acceleration, mut velocity, position, spline) in query.iter_mut() {
		**velocity += spline
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

pub fn update_current_spline(
	mut commands: Commands,
	mut query: Query<(
		Entity,
		&mut SplinePosition,
		&mut Spline,
		&mut SplineEdge,
		&SplineGraph,
	)>,
) {
	for (entity, mut position, mut spline, mut edge, graph) in query.iter_mut()
	{
		if **position >= 0. && **position <= 1. {
			continue;
		}
		let next_edge = match graph.get_current_spline(&edge, **position) {
			Some(value) => value,
			None => {
				commands
					.entity(entity)
					.remove::<SplinePosition>()
					.remove::<SplineVelocity>()
					.remove::<SplineEdge>();
				// .remove::<SplineGraph>();
				//TODO add velocity with tangent
				continue;
			}
		};
		*edge = next_edge;
		*spline = edge.spline;
		*position = SplinePosition(position.0 % 1.);
	}
}

pub fn update_spline_position(
	mut query: Query<(&mut SplinePosition, &SplineVelocity, &Spline)>,
	time: Res<Time>,
) {
	for (mut position, velocity, spline) in query.iter_mut() {
		**position += **velocity * time.delta_seconds();
		// println!("pos: {}, velocity: {}", **position, **velocity);
	}
}
pub fn update_transform_position(
	mut query: Query<(&mut Transform, &SplinePosition, &Spline)>,
) {
	for (mut transform, position, spline) in query.iter_mut() {
		transform.translation = spline.position(**position);
	}
}
