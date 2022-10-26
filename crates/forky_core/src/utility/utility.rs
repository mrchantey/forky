use bevy::{
	prelude::*,
	window::{PresentMode, WindowDescriptor},
};
use bevy_inspector_egui::WorldInspectorPlugin;

pub fn run() {
	App::new()
		.insert_resource(WindowDescriptor {
			width: 500.,
			height: 500.,
			title: "MagPie".to_string(),
			decorations: true,
			cursor_visible: true,
			cursor_locked: false,
			present_mode: PresentMode::AutoVsync,
			position: WindowPosition::At(Vec2::new(-1440., 0.)),
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(WorldInspectorPlugin::new())
		.run();
}
