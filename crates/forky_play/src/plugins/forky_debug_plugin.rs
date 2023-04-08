use crate::*;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;


pub struct ForkyDebugPlugin {
	debug_cameras: bool,
}

impl ForkyDebugPlugin {
	pub fn without_debug_cameras() -> Self {
		Self {
			debug_cameras: false,
			..default()
		}
	}
}

impl Default for ForkyDebugPlugin {
	fn default() -> Self {
		Self {
			debug_cameras: true,
		}
	}
}

impl Plugin for ForkyDebugPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(Msaa::default())
			// .insert_resource(ClearColor(Color::NAVY))
			.add_plugin(plugins::CustomDefaultPlugin)
			.add_plugin(input::InputPlugin)
			.add_plugin(camera::CameraPlugin)
			.add_plugin(materials::ForkyMaterialPlugin)
			.add_system(bevy::window::close_on_esc)
			.add_startup_system(utility::spawn_default_lights)
			.__();

		if self.debug_cameras {
			app.add_plugin(camera::DebugCameraPlugin);
		}
		if cfg!(debug_assertions) {
			app.__()
				.add_plugin(WorldInspectorPlugin::default().run_if(
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
				.add_plugin(debug::GridPlugin)
				// .add_system(toggle_inspector_on_keypress)
				.__();
		}
	}
}
