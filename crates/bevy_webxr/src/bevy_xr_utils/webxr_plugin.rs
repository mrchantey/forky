use bevy::prelude::*;
use bevy::render::RenderApp;

use super::*;

pub struct ClearGraphPlugin;

impl Plugin for ClearGraphPlugin {
	fn build(&self, app: &mut App) {

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
