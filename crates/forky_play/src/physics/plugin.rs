use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone)]
pub struct PhysicsPlugin {
	gravity: Vec3,
}

impl Default for PhysicsPlugin {
	fn default() -> Self {
		Self {
			gravity: Vec3::new(0., -9.8, 0.),
		}
	}
}


impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			.insert_resource(RapierConfiguration {
				gravity: self.gravity,
				..default()
			})
			.add_system(
				update_kinematic_bodies.in_base_set(CoreSet::PostUpdate),
			)
			.__();
		if cfg!(debug_assertions) {
			app.__()
				//
				.add_plugin(RapierDebugRenderPlugin::default())
				.__();
		}
	}
}
