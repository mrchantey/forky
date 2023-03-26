use crate::*;
use bevy::{log::LogPlugin, prelude::*, window::PresentMode, winit::*};
use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub struct ForkyFullPlugin;

impl Plugin for ForkyFullPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugin(plugins::ForkyDebugPlugin)
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			.insert_resource(RapierConfiguration {
				gravity: Vec3::new(0., -9.8, 0.),
				..default()
			})
			.add_plugin(EasingsPlugin)
			// .add_startup_system(utility::surrender_focus)
			// .add_startup_system(base::spawn_lights)
			//MY PLUGINS
			.add_system(animation::pose_lerp_animator)
			.__();
		if cfg!(debug_assertions) {
			app.__().add_plugin(RapierDebugRenderPlugin::default()).__();
		}
	}
}
