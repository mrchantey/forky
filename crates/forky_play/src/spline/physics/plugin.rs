use super::*;
use crate::*;
use bevy::prelude::*;

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
