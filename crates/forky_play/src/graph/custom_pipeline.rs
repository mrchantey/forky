use bevy::{
	core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
	prelude::*,
	render::{
		render_resource::{
			BindGroup, CachedRenderPipelineId, FragmentState, PipelineCache,
			RenderPipelineDescriptor,
		},
		renderer::RenderDevice,
		view::ViewTarget,
	},
};
use wgpu::{
	AddressMode, BindGroupDescriptor, BindGroupEntry,
	BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource,
	BindingType, BufferBinding, BufferBindingType, ColorTargetState,
	ColorWrites, FilterMode, MultisampleState, PrimitiveState,
	SamplerBindingType, SamplerDescriptor, ShaderStages, TextureFormat,
	TextureSampleType, TextureViewDimension,
};


#[derive(Resource)]
pub struct CustomPipeline {
	// pub bind_group: BindGroup,
	pub id: CachedRenderPipelineId,
}

impl FromWorld for CustomPipeline {
	fn from_world(world: &mut World) -> Self {
		let render_device = world.resource::<RenderDevice>();

		// let sampler = render_device.create_sampler(&SamplerDescriptor {
		// 	min_filter: FilterMode::Linear,
		// 	mag_filter: FilterMode::Linear,
		// 	address_mode_u: AddressMode::ClampToEdge,
		// 	address_mode_v: AddressMode::ClampToEdge,
		// 	..Default::default()
		// });
		let bind_group_layout = render_device.create_bind_group_layout(
			&BindGroupLayoutDescriptor {
				label: Some("custom_pipeline_bind_group_layout"),
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: BindingType::Buffer {
							ty: BufferBindingType::Uniform,
							// has_dynamic_offset: true,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					BindGroupLayoutEntry {
						binding: 1,
						visibility: ShaderStages::FRAGMENT,
						ty: BindingType::Texture {
							sample_type: TextureSampleType::Float {
								filterable: true,
							},
							view_dimension: TextureViewDimension::D2,
							multisampled: false,
						},
						count: None,
						// count: NonZeroU32::new(MAX_TEXTURE_COUNT as u32),
					},
					// // @group(1) @binding(1) var nearest_sampler: sampler;
					BindGroupLayoutEntry {
						binding: 2,
						visibility: ShaderStages::FRAGMENT,
						ty: BindingType::Sampler(SamplerBindingType::Filtering),
						count: None,
					},
				],
			},
		);

		let shader =
			world.resource::<AssetServer>().load("shaders/simple.wgsl");
		let mut pipeline_cache = world.resource_mut::<PipelineCache>();

		let id =
			pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
				label: Some("custom_pipeline".into()),
				layout: Some(vec![bind_group_layout.clone()]),
				vertex: fullscreen_shader_vertex_state(),
				fragment: Some(FragmentState {
					shader: shader,
					shader_defs: vec![],
					entry_point: "fragment".into(),
					targets: vec![Some(ColorTargetState {
						// format: ViewTarget::TEXTURE_FORMAT_HDR,
						format: TextureFormat::Rgba8UnormSrgb,
						blend: None,
						write_mask: ColorWrites::ALL,
					})],
				}),
				primitive: PrimitiveState::default(),
				depth_stencil: None,
				// multisample: MultisampleState::default(),
				multisample: MultisampleState {
					count: 4,
					..default()
				},
				// push_constant_ranges: Vec::new(),
			});

		// println!("yes, created id: {:?}", id);

		CustomPipeline {
			id,
			// bind_group
		}
	}
}
