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
	BindingType, BlendState, BufferBinding, BufferBindingType,
	ColorTargetState, ColorWrites, FilterMode, MultisampleState,
	PrimitiveState, SamplerBindingType, SamplerDescriptor, ShaderStages,
	TextureFormat, TextureSampleType, TextureViewDimension,
};


#[derive(Resource)]
pub struct BlitPipeline {
	// pub bind_group: BindGroup,
	pub id: CachedRenderPipelineId,
}

impl FromWorld for BlitPipeline {
	fn from_world(world: &mut World) -> Self {
		let render_device = world.resource::<RenderDevice>();

		let bind_group_layout = render_device.create_bind_group_layout(
			&BindGroupLayoutDescriptor {
				label: Some("blit_pipeline_bind_group_layout"),
				entries: &[
					BindGroupLayoutEntry {
						binding: 0,
						visibility: ShaderStages::FRAGMENT,
						ty: BindingType::Texture {
							sample_type: TextureSampleType::Float {
								filterable: true,
							},
							view_dimension: TextureViewDimension::D2,
							multisampled: false,
						},
						count: None,
					},
					BindGroupLayoutEntry {
						binding: 1,
						visibility: ShaderStages::FRAGMENT,
						ty: BindingType::Sampler(SamplerBindingType::Filtering),
						count: None,
					},
				],
			},
		);

		let shader = world.resource::<AssetServer>().load("shaders/blit.wgsl");
		let mut pipeline_cache = world.resource_mut::<PipelineCache>();

		let id =
			pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
				label: Some("blit_pipeline".into()),
				layout: vec![bind_group_layout.clone()],
				vertex: fullscreen_shader_vertex_state(),
				fragment: Some(FragmentState {
					shader: shader,
					shader_defs: vec![],
					entry_point: "fragment".into(),
					targets: vec![Some(ColorTargetState {
						// format: ViewTarget::TEXTURE_FORMAT_HDR,
						format: TextureFormat::Rgba8Unorm,
						// format: TextureFormat::Rgba8UnormSrgb,
						// blend: Some(BlendState::REPLACE),
						blend: None,
						write_mask: ColorWrites::ALL,
					})],
				}),
				primitive: PrimitiveState::default(),
				depth_stencil: None,
				// multisample: MultisampleState::default(),
				multisample: MultisampleState {
					count: 1,
					// count: 4,
					..default()
				},
				push_constant_ranges: Vec::new(),
			});

		BlitPipeline {
			id,
			// bind_group
		}
	}
}
