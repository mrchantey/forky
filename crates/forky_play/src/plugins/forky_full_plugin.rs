use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_easings::EasingsPlugin;

#[derive(Default,Clone)]
pub struct ForkyFullPlugin {
	pub debug_plugin: ForkyDebugPlugin,
	pub physics_plugin: physics::PhysicsPlugin,
	pub tool_plugin: tool::ToolPlugin,
}

impl Plugin for ForkyFullPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugin(self.debug_plugin.clone())
			.add_plugin(self.physics_plugin.clone())
			.add_plugin(self.tool_plugin.clone())
			.add_plugin(EasingsPlugin)
			// .add_startup_system(utility::surrender_focus)
			// .add_startup_system(base::spawn_lights)
			//MY PLUGINS
			.add_system(animation::pose_lerp_animator)
			.__();
	}
}
