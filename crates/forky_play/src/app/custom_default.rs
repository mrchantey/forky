use bevy::{
	log::LogPlugin,
	prelude::*,
	window::{PresentMode, WindowDescriptor},
};

use crate::utility;


pub struct CustomDefaultPlugin;

impl Plugin for CustomDefaultPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(Color::NAVY))
			.add_plugins(
				DefaultPlugins
					.set(WindowPlugin {
						window: WindowDescriptor {
							width: 1000.,
							height: 800.,
							title: "Forky".to_string(),
							decorations: true,
							cursor_visible: true,
							cursor_grab_mode:
								bevy::window::CursorGrabMode::None,
							resizable: true,
							// return:true,
							// winit
							present_mode: PresentMode::AutoVsync,
							position: WindowPosition::At(Vec2::new(-1440., 0.)),
							..Default::default()
						},
						..Default::default()
					})
					.set(LogPlugin {
						// filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
						level: bevy::log::Level::WARN,
						..Default::default()
					}),
			)
			.add_system(bevy::window::close_on_esc);
	}
}
