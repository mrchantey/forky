use bevy::prelude::*;
use bevy::render::RenderApp;

use super::*;

pub struct CustomPipelinePlugin;

impl Plugin for CustomPipelinePlugin {
	fn build(&self, app: &mut App) {
		let blit_source = create_blit_source(app);


		let render_app = app.get_sub_app_mut(RenderApp).unwrap();
		render_app
			.init_resource::<CustomPipeline>()
			.insert_resource(blit_source);

		let my_node = CustomPipelineNode::new(&mut render_app.world);

		insert_final_node(
			render_app,
			my_node,
			"custom_pipeline_pass",
			CustomPipelineNode::IN_VIEW,
		);
	}
}
