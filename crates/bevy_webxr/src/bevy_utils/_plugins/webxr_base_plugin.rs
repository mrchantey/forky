use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	log::LogPlugin,
	prelude::*,
	render::{
		camera::{camera_system, CameraProjectionPlugin},
		view::{update_frusta, VisibilitySystems},
	},
	transform::TransformSystem,
	window::{CompositeAlphaMode, WindowResolution},
	winit::WinitPlugin,
};

use crate::*;

pub struct WebXrBasePlugin;
#[rustfmt::skip]
impl Plugin for WebXrBasePlugin {
	fn build(&self, app: &mut App) {
		set_panic_hook();
		xr_utils::create_default_canvas().unwrap();

		app.__()
			.add_plugin(bevy_web_asset::WebAssetPlugin::default())
			.add_plugins(
				DefaultPlugins
					.set(WindowPlugin {
						primary_window: Some(Window {
							title: "Bevy WebXR Demo".into(),
							canvas: Some(xr_utils::BEVY_CANVAS_QUERY.into()),
							..Default::default()
						}),
						..Default::default()
					})
					.set(LogPlugin {
						// filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
						level: bevy::log::Level::WARN,
						..Default::default()
					})
					.build()
					.disable::<AssetPlugin>()//use webassetplugin
			)
			.add_plugin(bevy_utils::RawProjectionPlugin)
			.add_plugin(MaterialPlugin::<bevy_utils::UnlitMaterial>::default())
			.add_plugin(MaterialPlugin::<bevy_utils::UnlitTextureMaterial>::default())
			// .add_system(bevy_utils::replace_standard_material.in_base_set(CoreSet::PostUpdate))
			.__();
	}
}
