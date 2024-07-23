use crate::*;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::WindowResolution;

#[derive(Debug, Clone)]
pub struct CustomDefaultPlugin {
	pub custom_canvas: bool,
}

impl Default for CustomDefaultPlugin {
	fn default() -> Self {
		Self {
			custom_canvas: false,
		}
	}
}

impl Plugin for CustomDefaultPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugins(
				DefaultPlugins
					.set(AssetPlugin {
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
							canvas: if self.custom_canvas {
								Some(
									"canvas[data-bevy=\"primary_window\"]"
										.to_string(),
								)
							} else {
								None
							},
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
