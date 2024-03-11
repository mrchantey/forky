use super::*;
use crate::*;
use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct ForkyFullPlugin {
	pub debug_plugin: ForkyDebugPlugin,
	pub physics_plugin: physics::PhysicsPlugin,
	pub tool_plugin: tool::ToolPlugin,
}

impl Plugin for ForkyFullPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugins(self.debug_plugin.clone())
			.add_plugins(self.physics_plugin.clone())
			.add_plugins(self.tool_plugin.clone())
			// .add_systems(Startup, utility::surrender_focus)
			// .add_systems(Startup, base::spawn_lights)
			//MY PLUGINS
			.add_systems(Update, animation::pose_lerp_animator)
			.__();
	}
}
