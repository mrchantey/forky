use super::*;
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

#[derive(Reflect, AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f1b849ae-a498-4059-9e46-16b7e3f66722"]
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
