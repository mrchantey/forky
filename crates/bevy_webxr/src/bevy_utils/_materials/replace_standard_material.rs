use crate::*;
use bevy::prelude::*;




pub fn replace_standard_material(
	mut commands: Commands,
	mut standard_materials: ResMut<Assets<StandardMaterial>>,
	mut unlit_texture_materials: ResMut<
		Assets<bevy_utils::UnlitTextureMaterial>,
	>,
	query: Query<(Entity, &Handle<StandardMaterial>)>,
) {
	for (entity, standard_handle) in query.iter() {
		let standard_mat = match standard_materials.get(standard_handle) {
			Some(value) => value,
			None => continue,
		};
		let unlit_handle =
			unlit_texture_materials.add(bevy_utils::UnlitTextureMaterial {
				color_texture: standard_mat.base_color_texture.clone(),
				..default()
			});
		standard_materials.remove(standard_handle);
		// mat.base_color_texture
		commands
			.entity(entity)
			.remove::<Handle<StandardMaterial>>()
			.insert(unlit_handle);
	}
}
