use super::*;
use crate::forky_shader;
use bevy::prelude::*;

const SHADER_HANDLE_OFFSET_INTERNAL: u128 = 7_000_000_000_000_000;
pub const SHADER_HANDLE_OFFSET: u128 = SHADER_HANDLE_OFFSET_INTERNAL + 1000;

const SHADER_UTILITY: ForkyShader =
	forky_shader!("utility", SHADER_HANDLE_OFFSET_INTERNAL + 0)
		.mode(LoadMode::Inline);
const SHADER_NOISE: ForkyShader =
	forky_shader!("noise", SHADER_HANDLE_OFFSET_INTERNAL + 2)
		.mode(LoadMode::Inline);
const SHADER_SDF2: ForkyShader =
	forky_shader!("sdf2", SHADER_HANDLE_OFFSET_INTERNAL + 2)
		.mode(LoadMode::Inline);
const SHADER_SDF3: ForkyShader =
	forky_shader!("sdf3", SHADER_HANDLE_OFFSET_INTERNAL + 3)
		.mode(LoadMode::Inline);
pub const SHADER_UNLIT: ForkyShader =
	forky_shader!("unlit", SHADER_HANDLE_OFFSET_INTERNAL + 100)
		.mode(LoadMode::Internal);
pub const SHADER_UNLIT_TEXTURE: ForkyShader =
	forky_shader!("unlit_texture", SHADER_HANDLE_OFFSET_INTERNAL + 102)
		.mode(LoadMode::Internal);
pub const SHADER_UV: ForkyShader =
	forky_shader!("uv", SHADER_HANDLE_OFFSET_INTERNAL + 101)
		.mode(LoadMode::Internal);

const SHADERS: [ForkyShader; 7] = [
	SHADER_UTILITY,
	SHADER_NOISE,
	SHADER_SDF2,
	SHADER_SDF3,
	SHADER_UNLIT,
	SHADER_UNLIT_TEXTURE,
	SHADER_UV,
];

pub fn try_inline_shaders(app: &mut App) {
	for shader in SHADERS.iter() {
		shader.try_load_inline(app);
	}
}
