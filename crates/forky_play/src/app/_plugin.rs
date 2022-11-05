use crate::*;
use bevy::{
	log::LogSettings,
	prelude::*,
	window::{PresentMode, WindowDescriptor},
	winit::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
pub struct ForkyPlugin;

pub fn init() -> App {
	let mut app = App::new();
	app.add_plugin(ForkyPlugin);
	app
}
impl Plugin for ForkyPlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.insert_resource(Msaa::default())
			.insert_resource(LogSettings {
				// filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
				level: bevy::log::Level::WARN,
				..Default::default()
			})
			.add_plugin(input::DebugCameraPlugin)
			.insert_resource(WinitSettings {
				//SHOULD BE IN DEBUG MODE ONLY
				return_from_run: true,
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
			.add_plugin(RapierDebugRenderPlugin::default())
			.insert_resource(RapierConfiguration {
				gravity: Vec3::new(0., -9.8, 0.),
				..default()
			})
			.add_plugin(debug::GridPlugin)
			// .add_startup_system(utility::surrender_focus)
			.add_system(bevy::window::close_on_esc)
			.add_startup_system(base::spawn_lights)
			// .add_plugin(WorldInspectorPlugin::new())
			.forky();
	}
}
