use super::*;
use crate::*;
use bevy::prelude::*;

pub struct ForkyMaterialPlugin;

impl Plugin for ForkyMaterialPlugin {
	fn build(&self, app: &mut App) {
		load_shaders(app);
		app.__()
			.add_plugins(MaterialPlugin::<UvMaterial>::default())
			.add_plugins(MaterialPlugin::<UnlitMaterial>::default())
			.__();
	}
}
