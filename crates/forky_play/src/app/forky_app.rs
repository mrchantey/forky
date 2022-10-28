use crate::*;
use bevy::{
	prelude::*,
	window::{PresentMode, WindowDescriptor},
	winit::*, log::LogSettings,
};
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct ForkyApp {}

impl ForkyApp {
	pub fn init() -> App {
		let mut app = App::new();
		app
		.insert_resource(LogSettings {
			// filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
			level: bevy::log::Level::WARN,
			..Default::default()
		})
		.insert_resource(WinitSettings {
			//SHOULD BE IN DEBUG MODE ONLY
			return_from_run: true,
			..default()
		})
		.insert_resource(WindowDescriptor {
			width: 500.,
			height: 500.,
			title: "Forky".to_string(),
			decorations: true,
			cursor_visible: true,
			cursor_locked: false,
			resizable:true,
			// return:true,
			// winit
			present_mode: PresentMode::AutoVsync,
			position: WindowPosition::At(Vec2::new(-1440., 0.)),
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(surrender_focus)
		.add_system(bevy::window::close_on_esc)
		.add_plugin(WorldInspectorPlugin::new());
		app
	}
	pub fn run() -> App {
		let mut app = Self::init();
		app.run();
		app
	}
}
