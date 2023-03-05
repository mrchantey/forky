use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{
	Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ViewTarget};
use std::sync::Mutex;

use super::*;
use crate::*;

pub struct BlitNode;

impl Node for BlitNode {
	fn run(
		&self,
		_graph: &mut RenderGraphContext,
		render_context: &mut RenderContext,
		world: &World,
	) -> Result<(), NodeRunError> {
		let target = world.get_resource::<bevy_xr_utils::BlitTarget>();
		let pipeline_cache = world.resource::<PipelineCache>();
		let custom_pipeline = world.resource::<BlitPipeline>();
		let pipeline = pipeline_cache.get_render_pipeline(custom_pipeline.id);

		if let None = pipeline {
			return Ok(());
		}
		let pipeline = pipeline.unwrap();

		if let None = target {
			return Ok(());
		}
		let target = target.unwrap();

		let images = world.get_resource::<RenderAssets<Image>>().unwrap();
		let src = images.get(&target.src).unwrap();
		// let dest = images.get(&self.image_handle.dest).unwrap();
		// let src:wgpu::Texture = src.texture.into();
		// src.value.
		let size = Extent3d {
			width: target.width,
			height: target.height,
			..default()
		};

		let dest_view = &target
			.dest
			.create_view(&wgpu::TextureViewDescriptor::default());
		// let dest_view = &target.dest.create_view(&TextureViewDescriptor {
		// 	label: Some("Blit Target"),
		// 	format: Some(wgpu::TextureFormat::Rgba8Unorm),
		// 	// format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
		// 	dimension: Some(wgpu::TextureViewDimension::D2),
		// 	aspect: TextureAspect::All,
		// 	..default()
		// });

		let device = &render_context.render_device();

		//TODO cache bind group
		let bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("custom_pipeline_bind_group"),
			layout: &pipeline.get_bind_group_layout(0),
			entries: &[
				BindGroupEntry {
					binding: 0,
					resource: BindingResource::TextureView(&src.texture_view),
				},
				BindGroupEntry {
					binding: 1,
					resource: BindingResource::Sampler(&src.sampler),
				},
			],
		});

		let pass_descriptor = RenderPassDescriptor {
			label: Some("Clear Pass"),
			color_attachments: &[Some(RenderPassColorAttachment {
				view: dest_view,
				// resolve_target: Some(dest_view),
				resolve_target: None,
				// resolve_target: Some(self.main_texture()),
				ops: Operations {
					// load: LoadOp::Load,
					// load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
					load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
					// load: wgpu::LoadOp::Clear(wgpu::Color::RED),
					store: true,
				},
			})],
			depth_stencil_attachment: None,
		};

		let mut pass = render_context
			.command_encoder()
			.begin_render_pass(&pass_descriptor);
		pass.set_pipeline(pipeline);
		// pass.set_viewport(0., 0., 100., 100., 0.0, 1.0);
		pass.set_bind_group(0, &bind_group, &[]);
		pass.draw(0..3, 0..1);
		// pass.

		// log!("ok! {}", target.frame_count);
		Ok(())
	}
}
