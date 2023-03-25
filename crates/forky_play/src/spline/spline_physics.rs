use crate::*;
use bevy::prelude::*;
use derive_deref::{Deref, DerefMut};

use super::*;

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
	mut query_force: Query<(
		&physics::AccelerationForce,
		&mut SplineVelocity,
		&SplinePosition,
		&Spline,
	)>,
	time: Res<Time>,
) {
	for (acceleration, mut velocity, position, spline) in query_force.iter_mut()
	{
		**velocity += spline
			.acceleration(**position, **acceleration * time.delta_seconds());
	}
}

pub fn update_velocity_from_friction(
	mut query_force: Query<(&physics::Friction, &mut SplineVelocity)>,
	time: Res<Time>,
) {
	for (friction, mut velocity) in query_force.iter_mut() {
		let force = velocity.signum() * -1. * **friction * time.delta_seconds();
		**velocity += force;
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
