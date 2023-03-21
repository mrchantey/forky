use crate::*;
use bevy::{log::LogPlugin, prelude::*, window::PresentMode, winit::*};
use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_inspector_egui::{quick::WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

pub struct ForkyPlugin;

pub fn init() -> App {
	let mut app = App::new();
	app.add_plugin(ForkyPlugin);
	app
}
impl Plugin for ForkyPlugin {
	fn build(&self, app: &mut App) {
		let return_from_run =
			cfg!(all(debug_assertions, not(target_family = "wasm")));
		app.__()
			.insert_resource(Msaa::default())
			// .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
			// .add_plugins(DefaultPlugins)
			.add_plugin(input::DebugCameraPlugin)
			.insert_resource(WinitSettings {
				//SHOULD BE IN DEBUG MODE ONLY
				return_from_run,
				..default()
			})
			// .add_plugins(DefaultPlugins)
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			.insert_resource(RapierConfiguration {
				gravity: Vec3::new(0., -9.8, 0.),
				..default()
			})
			.add_plugin(EasingsPlugin)
			// .add_startup_system(utility::surrender_focus)
			.add_startup_system(base::spawn_lights)
			//MY PLUGINS
			.add_system(animation::pose_lerp_animator)
			.__();

		if cfg!(debug_assertions) {
			app.__()
				// .insert_resource(WorldInspectorParams {
				// 	enabled: false,
				// 	..default()
				// })
				.add_plugin(WorldInspectorPlugin::default().run_if(
					bevy::input::common_conditions::input_toggle_active(
						true,
						KeyCode::G,
					),
				))
				.add_plugin(RapierDebugRenderPlugin::default())
				.add_plugin(debug::GridPlugin)
				// .add_system(toggle_inspector_on_keypress)
				.__();
		}
	}
}
