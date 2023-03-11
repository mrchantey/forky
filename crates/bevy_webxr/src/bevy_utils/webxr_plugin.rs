use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::RenderApp;

use crate::*;
use web_sys::*;

pub struct WebXrPlugin;

impl Plugin for WebXrPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_startup_system(xr_utils::set_canvas_size)
			.add_startup_system(bevy_utils::setup_xr_cameras)
			.add_system(bevy_utils::resize_src_img)
			.add_system(bevy_utils::update_xr_tracking)
			.__();

		let mut render_app = app.get_sub_app_mut(RenderApp).unwrap();
		render_app
			.__()
			.init_resource::<bevy_utils::BlitPipeline>()
			.add_system(bevy_utils::update_framebuffer_texture)
			.__();

		bevy_utils::insert_node(
			render_app,
			bevy_utils::BlitNode,
			bevy_utils::BLIT_PASS,
			bevy_utils::END_MAIN_PASS,
			// bevy_utils::CLEAR_PASS,
			bevy_utils::FINAL_PASS,
		);
	}
}
