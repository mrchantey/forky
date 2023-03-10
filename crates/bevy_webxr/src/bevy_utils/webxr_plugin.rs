use bevy::prelude::*;
use bevy::render::RenderApp;

use crate::*;

pub struct WebXrPlugin;

impl Plugin for WebXrPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_startup_system(xr_utils::set_canvas_size)
			.add_startup_system(bevy_utils::setup_xr_cameras)
			.add_system(bevy_utils::resize_src_image)
			.__();

		let mut render_app = app.get_sub_app_mut(RenderApp).unwrap();
		render_app.init_resource::<bevy_utils::BlitPipeline>();

		// bevy_utils::insert_node(
		// 	render_app,
		// 	bevy_utils::ClearNode,
		// 	bevy_utils::CLEAR_PASS,
		// 	bevy_utils::END_MAIN_PASS,
		// 	bevy_utils::FINAL_PASS,
		// );
		bevy_utils::insert_node(
			render_app,
			bevy_utils::BlitNode,
			bevy_utils::BLIT_PASS,
			bevy_utils::END_MAIN_PASS,
			// bevy_utils::CLEAR_PASS,
			bevy_utils::FINAL_PASS,
		);
		// bevy_utils::insert_node(
		// 	render_app,
		// 	bevy_utils::ClearSrcNode,
		// 	bevy_utils::CLEAR_SRC_PASS,
		// 	bevy_utils::BLIT_PASS,
		// 	// bevy_utils::END_MAIN_PASS,
		// 	bevy_utils::FINAL_PASS,
		// );

		// let render_app = app.get_sub_app_mut(RenderApp).unwrap();
		// render_app.insert_resource(CustomClearColor {
		// 	r: 0.0,
		// 	g: 1.0,
		// 	b: 0.0,
		// 	a: 0.0,
		// });

		// let my_node = ClearNode::new(&mut render_app.world);

		// insert_final_node(
		// 	render_app,
		// 	my_node,
		// 	"clear_pass",
		// 	ClearNode::IN_VIEW,
		// );
	}
}
