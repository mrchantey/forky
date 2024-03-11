use super::SHADER_UNLIT_TEXTURE;
use bevy::pbr::MaterialPipeline;
use bevy::pbr::MaterialPipelineKey;
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::RenderPipelineDescriptor;
use bevy::render::render_resource::ShaderRef;
use bevy::render::render_resource::SpecializedMeshPipelineError;

// This is the struct that will be passed to your shader
#[derive(Debug, Clone, Asset, Reflect, AsBindGroup)]
pub struct UnlitTextureMaterial {
	#[uniform(0)]
	pub color: Color,
	/// Vec4(u,v,0,0) Bigger number = more tiles
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
	fn fragment_shader() -> ShaderRef { SHADER_UNLIT_TEXTURE.into() }

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
