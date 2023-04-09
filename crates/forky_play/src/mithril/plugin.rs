use super::*;
use crate::*;
use bevy::prelude::*;

pub struct MithrilPlugin;


impl Plugin for MithrilPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugin(plugins::ForkyFullPlugin {
				debug_plugin: plugins::ForkyDebugPlugin {
					debug_cameras: false,
					debug_grid: false,
				},
				tool_plugin: tool::ToolPlugin {
					settings: tool::InteractionSettings {
						intersect_normal: Vec3::Z_NEG,
						..default()
					},
				},
				..default()
			})
			.add_plugin(spline::SplinePlugin)
			.add_startup_system(spawn_camera)
			.add_startup_system(spawn_initial_graph)
			.add_startup_system(spawn_cart_settings)
			.add_system(spawn_carts)
			.__();
	}
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn(camera::FlyCameraBundle::forward());
}
