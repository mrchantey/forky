use crate::*;
use bevy::{prelude::*, render::RenderApp};

pub struct InvertPlugin;

impl Plugin for InvertPlugin {
	fn build(&self, app: &mut App) {
		let mut render_app = app.get_sub_app_mut(RenderApp).unwrap();
		render_app
			.__()
			.init_resource::<bevy_utils::BlitPipeline>()
			.__();
		bevy_utils::insert_node(
			render_app,
			bevy_utils::InvertNode,
			bevy_utils::BLIT_PASS,
			// bevy_utils::CLEAR_PASS,
			bevy_utils::FINAL_PASS,
			bevy_utils::EMPTY_PASS,
		);
	}
}
