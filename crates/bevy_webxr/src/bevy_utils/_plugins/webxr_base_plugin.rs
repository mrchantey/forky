use bevy::{log::LogPlugin, prelude::*};

use crate::*;

pub struct WebXrBasePlugin;
#[rustfmt::skip]
impl Plugin for WebXrBasePlugin {
	fn build(&self, app: &mut App) {
		set_panic_hook();
		xr_utils::create_default_canvas().unwrap();

		app.__()
			.add_plugins(bevy_web_asset::WebAssetPlugin::default())
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
			.add_plugins(bevy_utils::RawProjectionPlugin)
			.add_plugins(MaterialPlugin::<bevy_utils::UnlitMaterial>::default())
			.add_plugins(MaterialPlugin::<bevy_utils::UnlitTextureMaterial>::default())
			.__();
	}
}
// .add_systems(PostUpdate,bevy_utils::replace_standard_material)
