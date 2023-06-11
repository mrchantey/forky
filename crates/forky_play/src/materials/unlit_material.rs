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


pub const UNLIT_SHADER_HANDLE: HandleUntyped =
	HandleUntyped::weak_from_u64(Shader::TYPE_UUID, SHADER_ID_START + 1);



#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "51ecfdd0-839f-47d1-b5cc-edbccaec24ba"]
pub struct UnlitMaterial {
	#[uniform(0)]
	pub color: Color,
	pub alpha_mode: AlphaMode,
	// pub cull_mode: Option<Face>,
}

impl From<Color> for UnlitMaterial {
	fn from(color: Color) -> Self {
		Self {
			color,
			alpha_mode: AlphaMode::Opaque,
		}
	}
}

impl Material for UnlitMaterial {
	fn fragment_shader() -> ShaderRef { 
		if cfg!(feature = "shader_debug"){
			"shaders/unlit.wgsl".into()
		}else{
			UNLIT_SHADER_HANDLE.typed().into()
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
