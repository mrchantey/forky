use super::*;
use crate::forky_shader;
use bevy::{prelude::*, reflect::TypeUuid};

const SHADER_HANDLE_OFFSET_INTERNAL: u64 = 7_000_000_000_000_000;
pub const SHADER_HANDLE_OFFSET: u64 = SHADER_HANDLE_OFFSET_INTERNAL + 1000;

const SHADER_UTILITY: ForkyShader =
	forky_shader!("utility", SHADER_HANDLE_OFFSET_INTERNAL + 0).as_inline();
const SHADER_NOISE: ForkyShader =
	forky_shader!("noise", SHADER_HANDLE_OFFSET_INTERNAL + 2).as_inline();
const SHADER_SDF2: ForkyShader =
	forky_shader!("sdf2", SHADER_HANDLE_OFFSET_INTERNAL + 2).as_inline();
const SHADER_SDF3: ForkyShader =
	forky_shader!("sdf3", SHADER_HANDLE_OFFSET_INTERNAL + 3).as_inline();
pub const SHADER_UNLIT: ForkyShader =
	forky_shader!("unlit", SHADER_HANDLE_OFFSET_INTERNAL + 100).as_internal();
pub const SHADER_UV: ForkyShader =
	forky_shader!("uv", SHADER_HANDLE_OFFSET_INTERNAL + 101).as_internal();

const SHADERS: [ForkyShader; 6] = [
	SHADER_UTILITY,
	SHADER_NOISE,
	SHADER_SDF2,
	SHADER_SDF3,
	SHADER_UNLIT,
	SHADER_UV,
];

pub fn load_shaders(app: &mut App) {
	for shader in SHADERS.iter() {
		shader.try_load_inline(app);
	}
}
