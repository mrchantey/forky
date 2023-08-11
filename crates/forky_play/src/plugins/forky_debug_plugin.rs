use crate::*;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Clone)]
pub struct ForkyDebugPlugin {
	pub debug_cameras: bool,
	pub debug_grid: bool,
}

impl ForkyDebugPlugin {
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
		}
	}
}

impl Plugin for ForkyDebugPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(Msaa::default())
			// .insert_resource(ClearColor(Color::NAVY))
			.add_plugins(plugins::CustomDefaultPlugin)
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
						KeyCode::G,
					),
				))
				.insert_resource(WinitSettings {
					return_from_run: cfg!(all(
						debug_assertions,
						not(target_family = "wasm")
					)),
					..default()
				})
				// .add_systems(Update,toggle_inspector_on_keypress)
				.__();
			if self.debug_grid {
				app.add_plugins(debug::GridPlugin);
			}
		}
	}
}