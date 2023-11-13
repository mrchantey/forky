use crate::*;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum EulerPhysicsSet {
	Update,
}

pub struct EulerPhysicsPlugin;

#[rustfmt::skip]
impl Plugin for EulerPhysicsPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.configure_sets(PostUpdate, EulerPhysicsSet::Update)
			.add_systems(PostUpdate,(
				update_velocity_from_impulse,
				update_velocity_from_force,
				update_velocity_from_friction,
				update_position,
			).chain().in_set(EulerPhysicsSet::Update)
		)
			.__();
	}
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct AccelerationImpulse(pub Vec3);
#[derive(Default, Component, Deref, DerefMut)]
pub struct AccelerationForce(pub Vec3);

#[derive(Default, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);
#[derive(Default, Component, Deref, DerefMut)]
pub struct Friction(pub f32);


pub fn update_velocity_from_impulse(
	mut query_impulse: Query<(&mut AccelerationImpulse, &mut Velocity)>,
) {
	for (mut acceleration, mut velocity) in query_impulse.iter_mut() {
		**velocity += **acceleration;
		**acceleration = Vec3::ZERO;
	}
}
pub fn update_velocity_from_force(
	time: Res<Time<Real>>,
	mut query_force: Query<(&AccelerationForce, &mut Velocity)>,
) {
	for (acceleration, mut velocity) in query_force.iter_mut() {
		**velocity += **acceleration * time.delta_seconds();
	}
}
pub fn update_velocity_from_friction(
	time: Res<Time<Real>>,
	mut query_force: Query<(&Friction, &mut Velocity)>,
) {
	for (friction, mut velocity) in query_force.iter_mut() {
		let force =
			velocity.normalize() * -1. * **friction * time.delta_seconds();
		**velocity += force;
	}
}

pub fn update_position(
	time: Res<Time<Real>>,
	mut query: Query<(&mut Transform, &Velocity)>,
) {
	for (mut transform, velocity) in query.iter_mut() {
		transform.translation += **velocity * time.delta_seconds();
	}
}
