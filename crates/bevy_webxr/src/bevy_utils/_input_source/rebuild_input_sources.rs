use crate::{
	bevy_utils::{BevyXrInputSource, InputSourceHash},
	*,
};
use bevy::prelude::*;

use super::InputSourceAssetLookup;

// pub fn rebuild_input_sources_needed(
// 	events: EventReader<bevy_utils::InputSourceAdded>,
// ) -> bool {
// 	events.len() > 0
// }
pub fn remove_input_sources(
	mut commands: Commands,
	query: Query<(Entity, &InputSourceHash)>,
	mut events: EventReader<bevy_utils::InputSourceRemoved>,
) {
	for event in events.iter() {
		for (entity, hash) in query.iter() {
			if **hash == **event {
				// log!("deleting input {}",input_source.hash);
				commands.entity(entity).despawn();
			}
		}
	}
}
// pub fn rebuild_input_sources(
// 	mut commands: Commands,
// 	mut events: EventReader<bevy_utils::InputSourceAdded>,
// 	asset_server: Res<AssetServer>,
// 	input_source_asset_lookup: Res<InputSourceAssetLookup>,
// ) {
// 	for hash in events.iter() {
// 		if let Some(asset_path) = input_source_asset_lookup.get(&hash) {
// 			let handle: Handle<Scene> = asset_server.load(asset_path);
// 			// log!("rebuilding source {}", **hash);
// 			commands
// 				.spawn(SceneBundle {
// 					scene: handle,
// 					transform: Transform::from_xyz(0.0, 0.0, 0.0),
// 					..default()
// 				})
// 		}
// 	}
// }
pub fn rebuild_input_sources(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut events: EventReader<bevy_utils::InputSourceAdded>,
	mut materials: ResMut<Assets<bevy_utils::UnlitMaterial>>,
) {
	for hash in events.iter() {
		commands.spawn((
			MaterialMeshBundle {
				mesh: meshes.add(Mesh::from(shape::Box {
					min_x: -0.02,
					max_x: 0.02,
					min_y: -0.01,
					max_y: 0.01,
					min_z: -0.05,
					max_z: 0.05,
				})),
				material: materials.add(bevy_utils::UnlitMaterial {
					color: Color::YELLOW,
					..default()
				}),
				transform: Transform::from_xyz(0.0, 0., -0.3),
				..default()
			},
			InputSourceHash(**hash),
		));
	}
}
