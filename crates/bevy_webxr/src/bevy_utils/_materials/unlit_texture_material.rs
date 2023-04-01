use bevy::{
	pbr::{MaterialPipeline, MaterialPipelineKey},
	prelude::*,
	reflect::TypeUuid,
	render::{
		mesh::MeshVertexBufferLayout,
		render_resource::{
			AsBindGroup, RenderPipelineDescriptor, ShaderRef,
			SpecializedMeshPipelineError,
		},
	},
};

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "9c11a0a4-7f97-4564-9634-cf0c741ae170"]
pub struct UnlitTextureMaterial {
	#[uniform(0)]
	pub color: Color,
	#[uniform(1)]
	pub tiling: Vec4,
	#[texture(2)]
	#[sampler(3)]
	pub color_texture: Option<Handle<Image>>,
	pub alpha_mode: AlphaMode,
}

impl Default for UnlitTextureMaterial {
	fn default() -> Self {
		Self {
			tiling: Vec4::new(1., 1., 0., 0.),
			color: Color::WHITE,
			color_texture: None,
			alpha_mode: AlphaMode::Opaque,
		}
	}
}


impl Material for UnlitTextureMaterial {
	fn fragment_shader() -> ShaderRef { "shaders/unlit_texture.wgsl".into() }

	fn alpha_mode(&self) -> AlphaMode { self.alpha_mode }
	fn specialize(
		_pipeline: &MaterialPipeline<Self>,
		descriptor: &mut RenderPipelineDescriptor,
		_layout: &MeshVertexBufferLayout,
		_key: MaterialPipelineKey<Self>,
	) -> Result<(), SpecializedMeshPipelineError> {
		descriptor.primitive.cull_mode = None;
		Ok(())
	}
}
