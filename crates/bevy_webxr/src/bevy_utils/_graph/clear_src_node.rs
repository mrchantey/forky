use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{
	Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ExtractedWindows, ViewTarget};
use std::sync::Mutex;

use super::*;
use crate::*;

pub struct ClearSrcNode;

impl Node for ClearSrcNode {
	fn run(
		&self,
		_graph: &mut RenderGraphContext,
		render_context: &mut RenderContext,
		world: &World,
	) -> Result<(), NodeRunError> {
		let target = match world.get_resource::<bevy_utils::BlitTarget>() {
			Some(value) => value,
			None => return Ok(()),
		};

		let images = world.get_resource::<RenderAssets<Image>>().unwrap();
		let src = images.get(&target.src).unwrap();

		let pass_descriptor = RenderPassDescriptor {
			label: Some("clear_window_pass"),
			color_attachments: &[Some(RenderPassColorAttachment {
				view: &src.texture_view,
				resolve_target: None,
				ops: Operations {
					load: LoadOp::Clear(wgpu::Color::TRANSPARENT),
					store: true,
				},
			})],
			depth_stencil_attachment: None,
		};

		render_context
			.command_encoder()
			.begin_render_pass(&pass_descriptor);
		// render_context
		// 	.command_encoder()
		// 	.clear_texture(&src.texture, &ImageSubresourceRange::default());



		Ok(())
	}
}
