use super::*;
use crate::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.add_system_to_stage(CoreStage::PostUpdate, update_kinematic_bodies)
			.forky();
	}
}
