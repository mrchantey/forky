use super::*;
use bevy::pbr::MaterialPipeline;
use bevy::pbr::MaterialPipelineKey;
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::RenderPipelineDescriptor;
use bevy::render::render_resource::ShaderRef;
use bevy::render::render_resource::SpecializedMeshPipelineError;

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
pub struct UvMaterial {
	#[uniform(0)]
	pub color: Color,
	pub alpha_mode: AlphaMode,
}

impl Material for UvMaterial {
	fn fragment_shader() -> ShaderRef { SHADER_UV.into() }
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
