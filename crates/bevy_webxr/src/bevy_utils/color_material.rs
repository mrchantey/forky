use bevy::{
	prelude::*,
	reflect::TypeUuid,
	render::render_resource::{AsBindGroup, ShaderRef},
};

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct MyColorMaterial {
	#[uniform(0)]
	pub color: Color,
	pub alpha_mode: AlphaMode,
}

impl Material for MyColorMaterial {
	fn fragment_shader() -> ShaderRef { "shaders/color.wgsl".into() }

	fn alpha_mode(&self) -> AlphaMode { self.alpha_mode }
}
