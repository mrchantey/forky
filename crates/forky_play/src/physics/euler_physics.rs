use crate::*;
use bevy::prelude::*;
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum EulerPhysicsSet {
	Update,
}

pub struct EulerPhysicsPlugin;

#[rustfmt::skip]
impl Plugin for EulerPhysicsPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.configure_set(EulerPhysicsSet::Update
				.in_base_set(CoreSet::PostUpdate))
			.add_system(update_velocity_from_impulse
				.in_set(EulerPhysicsSet::Update)
			)
			.add_system(update_velocity_from_force
				.in_set(EulerPhysicsSet::Update)
				.after(update_velocity_from_impulse),
			)
			.add_system(update_position
				.in_set(EulerPhysicsSet::Update)
				.after(update_velocity_from_force),
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


pub fn update_velocity_from_impulse(
	mut query_impulse: Query<(&mut AccelerationImpulse, &mut Velocity)>,
) {
	for (mut acceleration, mut velocity) in query_impulse.iter_mut() {
		**velocity += **acceleration;
		**acceleration = Vec3::ZERO;
	}
}
pub fn update_velocity_from_force(
	mut query_force: Query<(&AccelerationForce, &mut Velocity)>,
	time: Res<Time>,
) {
	for (acceleration, mut velocity) in query_force.iter_mut() {
		**velocity += **acceleration * time.delta_seconds();
	}
}
pub fn update_position(
	mut query: Query<(&mut Transform, &Velocity)>,
	time: Res<Time>,
) {
	for (mut transform, velocity) in query.iter_mut() {
		transform.translation += **velocity * time.delta_seconds();
	}
}
