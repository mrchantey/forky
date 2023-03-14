use bevy::{
	prelude::*,
	reflect::TypeUuid,
	render::{render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError}, mesh::MeshVertexBufferLayout}, pbr::{MaterialPipeline, MaterialPipelineKey},
};

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "51ecfdd0-839f-47d1-b5cc-edbccaec24ba"]
pub struct UnlitMaterial {
	#[uniform(0)]
	pub color: Color,
	pub alpha_mode: AlphaMode,
}


impl Material for UnlitMaterial {
	fn fragment_shader() -> ShaderRef { "shaders/unlit.wgsl".into() }

	fn alpha_mode(&self) -> AlphaMode { self.alpha_mode }
	fn specialize(
		pipeline: &MaterialPipeline<Self>,
		descriptor: &mut RenderPipelineDescriptor,
		layout: &MeshVertexBufferLayout,
		key: MaterialPipelineKey<Self>,
	) -> Result<(), SpecializedMeshPipelineError> {
		descriptor.primitive.cull_mode = None;
		Ok(())
	}
}
