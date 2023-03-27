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
use wgpu::Face;


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
