use super::SHADER_ID_START;
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

pub const UV_SHADER_HANDLE: HandleUntyped =
	HandleUntyped::weak_from_u64(Shader::TYPE_UUID, SHADER_ID_START + 2);



#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f1b849ae-a498-4059-9e46-16b7e3f66722"]
pub struct UvMaterial {
	#[uniform(0)]
	pub color: Color,
	pub alpha_mode: AlphaMode,
}


impl Material for UvMaterial {
	// fn fragment_shader() -> ShaderRef { "shaders/uv.wgsl".into() }
	fn fragment_shader() -> ShaderRef {
		if cfg!(feature = "shader_debug") {
			"shaders/uv.wgsl".into()
		} else {
			UV_SHADER_HANDLE.typed().into()
		}
	}

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
