use crate::{
	materials::{UNLIT_SHADER_HANDLE, UV_SHADER_HANDLE},
	*,
};
use bevy::{asset::load_internal_asset, prelude::*};

pub const SHADER_ID_START: u64 = 7_000_000_000_000_000;

pub struct ForkyMaterialPlugin;

impl Plugin for ForkyMaterialPlugin {
	fn build(&self, app: &mut App) {
		if cfg!(not(feature = "shader_debug")) {
			println!("shader_debug enabled");
			load_internal_asset!(
				app,
				UNLIT_SHADER_HANDLE,
				"../../assets/shaders/unlit.wgsl",
				Shader::from_wgsl
			);
			load_internal_asset!(
				app,
				UV_SHADER_HANDLE,
				"../../assets/shaders/uv.wgsl",
				Shader::from_wgsl
			);
		}

		app.__()
			.add_plugin(MaterialPlugin::<materials::UvMaterial>::default())
			.add_plugin(MaterialPlugin::<materials::UnlitMaterial>::default())
			.__();
	}
}
