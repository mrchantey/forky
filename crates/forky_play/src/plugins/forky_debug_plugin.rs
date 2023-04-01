use crate::*;
use bevy::{
	prelude::*,
	winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;


pub struct ForkyDebugPlugin;

impl Plugin for ForkyDebugPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(Msaa::default())
			// .insert_resource(ClearColor(Color::NAVY))
			.add_plugin(plugins::CustomDefaultPlugin)
			.add_plugin(input::DebugCameraPlugin)
			.add_plugin(materials::ForkyMaterialPlugin)
			.add_system(bevy::window::close_on_esc)
			.add_startup_system(utility::spawn_default_lights)
			.__();
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
