use crate::*;
use bevy::{
	log::LogPlugin,
	prelude::*,
	window::{PresentMode, WindowResolution},
};

pub struct CustomDefaultPlugin;

impl Plugin for CustomDefaultPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugins(
				DefaultPlugins
					.set(AssetPlugin {
						watch_for_changes: true,
						// watch_for_changes: ChangeWatcher::with_delay(
						// 	Duration::from_millis(200),
						// )
						..Default::default()
					})
					.set(WindowPlugin {
						primary_window: Some(Window {
							resolution: WindowResolution::from((1000., 800.)),
							title: "Forky".into(),
							decorations: true,
							// cursor_visible: true,
							// cursor_grab_mode:
							// 	bevy::window::CursorGrabMode::None,
							resizable: true,
							// return:true,
							// winit
							present_mode: PresentMode::AutoVsync,
							position: WindowPosition::At(IVec2::new(-1440, 0)),
							canvas: Some(
								"canvas[data-bevy=\"primary_window\"]"
									.to_string(),
							),
							..Default::default()
						}),
						..Default::default()
					})
					.set(LogPlugin {
						// filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
						level: bevy::log::Level::WARN,
						..Default::default()
					}),
			)
			.__();
	}
}
