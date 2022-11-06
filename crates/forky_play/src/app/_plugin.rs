use crate::{utility::toggle_inspector_on_keypress, *};
use bevy::{
	log::LogSettings,
	prelude::*,
	window::{PresentMode, WindowDescriptor},
	winit::*,
};
use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
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
		app.forky()
			.insert_resource(ClearColor(Color::BLACK))
			.insert_resource(Msaa::default())
			.insert_resource(LogSettings {
				// filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
				level: bevy::log::Level::WARN,
				..Default::default()
			})
			.add_plugin(input::DebugCameraPlugin)
			.insert_resource(WinitSettings {
				//SHOULD BE IN DEBUG MODE ONLY
				return_from_run,
				..default()
			})
			.insert_resource(WindowDescriptor {
				width: 1000.,
				height: 800.,
				title: "Forky".to_string(),
				decorations: true,
				cursor_visible: true,
				cursor_locked: false,
				resizable: true,
				// return:true,
				// winit
				present_mode: PresentMode::AutoVsync,
				position: WindowPosition::At(Vec2::new(-1440., 0.)),
				..Default::default()
			})
			.add_plugins(DefaultPlugins)
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			.insert_resource(RapierConfiguration {
				gravity: Vec3::new(0., -9.8, 0.),
				..default()
			})
			.add_plugin(EasingsPlugin)
			// .add_startup_system(utility::surrender_focus)
			.add_system(bevy::window::close_on_esc)
			.add_startup_system(base::spawn_lights)
			//MY PLUGINS
			.add_system(animation::pose_lerp_animator)
			.forky();

		if cfg!(debug_assertions) {
			app.forky()
				.insert_resource(WorldInspectorParams {
					enabled: false,
					..default()
				})
				.add_plugin(WorldInspectorPlugin::new())
				.add_plugin(RapierDebugRenderPlugin::default())
				.add_plugin(debug::GridPlugin)
				.add_system(toggle_inspector_on_keypress)
				.forky();
		}
	}
}
