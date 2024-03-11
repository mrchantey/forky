use super::*;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;

#[derive(Asset, Reflect, AsBindGroup,  Debug, Clone, Default)]
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
	fn fragment_shader() -> ShaderRef { SHADER_UNLIT.into() }
	fn alpha_mode(&self) -> AlphaMode { self.alpha_mode }
	// fn specialize(
	// 	_pipeline: &MaterialPipeline<Self>,
	// 	descriptor: &mut RenderPipelineDescriptor,
	// 	_layout: &MeshVertexBufferLayout,
	// 	_key: MaterialPipelineKey<Self>,
	// ) -> Result<(), SpecializedMeshPipelineError> {
	// 	descriptor.primitive.cull_mode = None;
	// 	Ok(())
	// }
}
