use super::CustomDefaultPlugin;
use crate::*;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Clone)]
pub struct ForkyDebugPlugin {
	pub debug_cameras: bool,
	pub debug_grid: bool,
	pub custom_default: CustomDefaultPlugin,
}

impl ForkyDebugPlugin {
	pub fn with_custom_canvas(mut self) -> Self {
		self.custom_default.custom_canvas = true;
		self
	}
	pub fn without_debug_cameras(mut self) -> Self {
		self.debug_cameras = false;
		self
	}
	pub fn without_debug_grid(mut self) -> Self {
		self.debug_grid = false;
		self
	}
}

impl Default for ForkyDebugPlugin {
	fn default() -> Self {
		Self {
			debug_grid: true,
			debug_cameras: true,
			custom_default: CustomDefaultPlugin::default(),
		}
	}
}

impl Plugin for ForkyDebugPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(Msaa::default())
			// .insert_resource(ClearColor(Color::NAVY))
			.add_plugins(self.custom_default.clone())
			.add_plugins(input::InputPlugin)
			.add_plugins(camera::CameraPlugin)
			.add_plugins(materials::ForkyMaterialPlugin)
			.add_systems(Update, bevy::window::close_on_esc)
			.add_systems(Startup, utility::spawn_default_lights)
			.__();

		if self.debug_cameras {
			app.add_plugins(camera::DebugCameraPlugin);
		}
		if cfg!(debug_assertions) {
			app.__()
				.add_plugins(WorldInspectorPlugin::default().run_if(
					bevy::input::common_conditions::input_toggle_active(
						false,
						KeyCode::KeyG,
					),
				))
				.insert_resource(WinitSettings { ..default() })
				// .add_systems(Update,toggle_inspector_on_keypress)
				.__();
			if self.debug_grid {
				app.add_plugins(debug::GridPlugin);
			}
		}
	}
}
