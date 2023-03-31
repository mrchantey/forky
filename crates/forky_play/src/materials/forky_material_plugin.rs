use crate::*;
use bevy::prelude::*;


pub struct ForkyMaterialPlugin;

impl Plugin for ForkyMaterialPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugin(MaterialPlugin::<materials::UvMaterial>::default())
			.add_plugin(MaterialPlugin::<materials::UnlitMaterial>::default())
			.__();
	}
}
