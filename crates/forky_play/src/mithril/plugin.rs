use super::*;
use crate::*;
use bevy::prelude::*;

pub struct MithrilPlugin;


impl Plugin for MithrilPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugins(plugins::ForkyFullPlugin {
				debug_plugin: plugins::ForkyDebugPlugin {
					debug_cameras: false,
					debug_grid: false,
					..default()
				},
				tool_plugin: tool::ToolPlugin {
					settings: tool::InteractionSettings {
						intersect_normal: Vec3::Z_NEG,
						..default()
					},
				},
				..default()
			})
			.add_plugins(spline::SplinePlugin)
			.add_systems(Startup, spawn_camera)
			.add_systems(Startup, spawn_initial_graph)
			.add_systems(Startup, spawn_cart_settings)
			.add_systems(PreUpdate, spawn_carts)
			.__();
	}
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn(camera::FlyCameraBundle::forward());
}
