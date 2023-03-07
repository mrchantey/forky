use bevy::prelude::*;
use bevy::render::render_graph::{
	Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ExtractedWindows, ViewTarget};

use crate::*;

// use super::CustomClearColor;




pub struct ClearNode;

impl Node for ClearNode {
	fn run(
		&self,
		_graph: &mut RenderGraphContext,
		render_context: &mut RenderContext,
		world: &World,
	) -> Result<(), NodeRunError> {
		// let len = self.query.iter_manual(world).len();
		// log!("rendering for {} views", len);
		// let mut i = 0;
		for (_, window) in world.resource::<ExtractedWindows>().iter() {
			let Some(swap_chain_texture) = &window.swap_chain_texture else {
					continue;
			};

			let pass_descriptor = RenderPassDescriptor {
				label: Some("clear_window_pass"),
				color_attachments: &[Some(RenderPassColorAttachment {
					view: swap_chain_texture,
					resolve_target: None,
					ops: Operations {
						load: LoadOp::Clear(wgpu::Color::GREEN),
						// load: LoadOp::Clear(wgpu::Color::TRANSPARENT),
						// load: LoadOp::Clear(wgpu::Color::BLACK),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			};

			render_context
				.command_encoder()
				.begin_render_pass(&pass_descriptor);
		}
		// for (i, target) in self.query.iter_manual(world).enumerate() {
		// 	if i == 0 {
		// 		continue;
		// 	}
		// 	let pass_descriptor = RenderPassDescriptor {
		// 		label: Some("Clear Pass"),
		// 		color_attachments: &[Some(target.get_color_attachment(
		// 			Operations {
		// 				// load: LoadOp::Load,
		// 				load: wgpu::LoadOp::Clear(Default::default()),
		// 				// load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
		// 				// load: wgpu::LoadOp::Clear(wgpu::Color {
		// 				// 	r: self.clear_color.r,
		// 				// 	g: self.clear_color.g,
		// 				// 	b: self.clear_color.b,
		// 				// 	a: self.clear_color.a,
		// 				// }),
		// 				store: true,
		// 			},
		// 		))],
		// 		depth_stencil_attachment: None,
		// 	};

		// 	// render_context.begin_tracked_render_pass(pass_descriptor);
		// 	render_context
		// 	.command_encoder()
		// 	.begin_render_pass(&pass_descriptor);
		// }
		Ok(())
	}
}
