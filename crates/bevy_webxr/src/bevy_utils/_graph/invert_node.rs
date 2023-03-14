use bevy::prelude::*;
use bevy::render::camera::ManualTextureViews;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{
	Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ExtractedWindows, ViewTarget};
use std::sync::Mutex;

use super::*;
use crate::bevy_utils::FramebufferTextureViewId;
use crate::*;

pub struct InvertNode;

impl Node for InvertNode {
	fn run(
		&self,
		_graph: &mut RenderGraphContext,
		render_context: &mut RenderContext,
		world: &World,
	) -> Result<(), NodeRunError> {
		let pipeline_cache = world.resource::<PipelineCache>();
		let custom_pipeline = world.resource::<BlitPipeline>();
		let pipeline =
			match pipeline_cache.get_render_pipeline(custom_pipeline.id) {
				Some(value) => value,
				None => return Ok(()),
			};
		let manual_texture_views = world.resource::<ManualTextureViews>();
		let id = world.resource::<FramebufferTextureViewId>();
		let view = manual_texture_views.get(&id).unwrap().0.clone();
		let device = &render_context.render_device();
		let sampler =
			device.create_sampler(&wgpu::SamplerDescriptor::default());

		//TODO cache bind group
		let bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("blit_bind_group"),
			layout: &pipeline.get_bind_group_layout(0),
			entries: &[
				BindGroupEntry {
					binding: 0,
					resource: BindingResource::TextureView(&view),
				},
				BindGroupEntry {
					binding: 1,
					resource: BindingResource::Sampler(&sampler),
				},
			],
		});

		let pass_descriptor = RenderPassDescriptor {
			label: Some("Clear Pass"),
			color_attachments: &[Some(RenderPassColorAttachment {
				view: &view,
				// resolve_target: Some(dest_view),
				resolve_target: None,
				ops: Operations {
					load: LoadOp::Load,
					store: true,
				},
			})],
			depth_stencil_attachment: None,
		};

		// let mut pass =
		// 	render_context.begin_tracked_render_pass(pass_descriptor);
		let mut pass = render_context
			.command_encoder()
			.begin_render_pass(&pass_descriptor);
		// pass.set_render_pipeline(pipeline);
		pass.set_pipeline(pipeline);
		// pass.set_viewport(0., 0., 100., 100., 0.0, 1.0);
		pass.set_bind_group(0, &bind_group, &[]);
		pass.draw(0..3, 0..1);

		Ok(())
	}
}
