use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::RenderApp;

use crate::*;
use web_sys::*;

pub struct WebXrPlugin;

#[rustfmt::skip]
impl Plugin for WebXrPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(bevy_utils::FramebufferTextureViewId(69))
			.add_system(bevy_utils::update_xr_resources
				.in_base_set(CoreSet::PreUpdate)
			)
			.add_system(bevy_utils::remove_xr_cameras
				.in_base_set(CoreSet::PreUpdate)
				.run_if(bevy_utils::cameras_need_rebuild)
				.after(bevy_utils::update_xr_resources)
			)
			.add_system(bevy_utils::setup_xr_cameras
				.in_base_set(CoreSet::PreUpdate)
				.run_if(bevy_utils::cameras_need_rebuild)
				.after(bevy_utils::remove_xr_cameras)
			)
			.add_system(bevy_utils::update_xr_cameras
				// .in_base_set(CoreSet::PreUpdate)
				.after(bevy_utils::setup_xr_cameras)
			)
			.add_startup_system(xr_utils::set_canvas_size)
			.add_system(bevy_utils::update_manual_texture_views)
			.__();
	}
}
