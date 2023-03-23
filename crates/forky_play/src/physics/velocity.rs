use crate::*;
use bevy::prelude::*;
use derive_deref::{Deref, DerefMut};


pub struct VelocityPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum VelocitySet {
	Update,
}
#[rustfmt::skip]
impl Plugin for VelocityPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.configure_set(VelocitySet::Update
				.in_base_set(CoreSet::PostUpdate))
			.add_system(update_velocity_from_impulse
				.in_set(VelocitySet::Update)
			)
			.add_system(update_velocity_from_force
				.in_set(VelocitySet::Update)
				.after(update_velocity_from_impulse),
			)
			.add_system(update_position
				.in_set(VelocitySet::Update)
				.after(update_velocity_from_force),
			);
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
		// **velocity = **velocity * 2. + **acceleration * time.delta_seconds().powf(2.) * 0.5;
		// **velocity += **acceleration * time.delta_seconds().powf(2.) * 0.5;
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
