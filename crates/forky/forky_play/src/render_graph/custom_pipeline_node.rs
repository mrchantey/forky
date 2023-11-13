// use super::BlitSource;
// use super::CustomPipeline;
use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
// use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::Node;
use bevy::render::render_graph::NodeRunError;
use bevy::render::render_graph::RenderGraphContext;
use bevy::render::render_graph::SlotInfo;
use bevy::render::render_graph::SlotType;
// use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::ExtractedView;
use bevy::render::view::ViewTarget;

pub struct CustomPipelineNode {
	query: QueryState<
		(&'static ViewTarget, &'static ExtractedCamera),
		With<ExtractedView>,
	>,
	// clear_color: CustomClearColor,
}


impl CustomPipelineNode {
	pub const IN_VIEW: &'static str = "view";

	pub fn new(world: &mut World) -> Self {
		Self {
			query: QueryState::new(world),
			// clear_color: CustomClearColor::default(),
		}
	}
}

impl Node for CustomPipelineNode {
	fn input(&self) -> Vec<SlotInfo> {
		vec![SlotInfo::new(CustomPipelineNode::IN_VIEW, SlotType::Entity)]
	}

	fn update(&mut self, world: &mut World) {
		self.query.update_archetypes(world);
		// self.clear_color = *world.get_resource::<CustomClearColor>().unwrap();
	}

	fn run(
		&self,
		_graph: &mut RenderGraphContext,
		_render_context: &mut RenderContext,
		_world: &World,
	) -> Result<(), NodeRunError> {
		todo!("upgrade to bevy 0.12");

		// let pipeline_cache = world.resource::<PipelineCache>();
		// let custom_pipeline = world.resource::<CustomPipeline>();
		// let pipeline = pipeline_cache.get_render_pipeline(custom_pipeline.id);

		// if let None = pipeline {
		// 	return Ok(());
		// }
		// let pipeline = pipeline.unwrap();

		// let col: [f32; 4] = [1., 1.0, 1.0, 1.0];

		// let device = &render_context.render_device();

		// let buffer =
		// 	device.create_buffer_with_data(&wgpu::util::BufferInitDescriptor {
		// 		label: Some("custom_pipeline_uniform_buffer"),
		// 		usage: wgpu::BufferUsages::UNIFORM
		// 			| wgpu::BufferUsages::COPY_DST,
		// 		contents: bytemuck::bytes_of(&col),
		// 	});

		// let blit_src = world.get_resource::<BlitSource>().unwrap();
		// let images = world.get_resource::<RenderAssets<Image>>().unwrap();
		// let src = images.get(&blit_src.src).unwrap();

		// let bind_group = device.create_bind_group(
		// 	Some("custom_pipeline_bind_group"),
		// 	&pipeline.get_bind_group_layout(0),
		// 	&[
		// 		BindGroupEntry {
		// 			binding: 0,
		// 			resource: BindingResource::Buffer(BufferBinding {
		// 				buffer: &buffer,
		// 				offset: 0,
		// 				size: None,
		// 			}),
		// 		},
		// 		BindGroupEntry {
		// 			binding: 1,
		// 			resource: BindingResource::TextureView(
		// 				&src.texture_view,
		// 				// view_target.main_texture(),
		// 			),
		// 		},
		// 		BindGroupEntry {
		// 			binding: 2,
		// 			resource: BindingResource::Sampler(&src.sampler),
		// 		},
		// 	],
		// );

		// for (target, camera) in self.query.iter_manual(world) {
		// 	if camera.order < 0 {
		// 		continue;
		// 	}

		// 	let pass_descriptor = RenderPassDescriptor {
		// 		label: Some("Clear Pass"),
		// 		color_attachments: &[Some(target.get_color_attachment(
		// 			Operations {
		// 				// load: LoadOp::Load,
		// 				load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
		// 				store: true,
		// 			},
		// 		))],
		// 		depth_stencil_attachment: None,
		// 	};

		// 	let mut pass = render_context
		// 		.command_encoder()
		// 		.begin_render_pass(&pass_descriptor);
		// 	pass.set_pipeline(pipeline);
		// 	pass.set_bind_group(0, &bind_group, &[]);
		// 	pass.draw(0..3, 0..1);
		// }
		// Ok(())
	}
}
