use bevy::{
	prelude::*, reflect::TypeUuid, render::render_resource::{AsBindGroup, ShaderRef},
};


// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
	#[uniform(0)]
	pub color: Color,
	#[texture(1)]
	#[sampler(2)]
	pub color_texture: Option<Handle<Image>>,
	pub alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
	fn fragment_shader() -> ShaderRef { "shaders/custom_material.wgsl".into() }

	fn alpha_mode(&self) -> AlphaMode { self.alpha_mode }
}
