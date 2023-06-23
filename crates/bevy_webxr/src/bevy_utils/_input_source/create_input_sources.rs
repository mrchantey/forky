use super::BevyXrInputSource;
use crate::*;
use bevy::{prelude::*, utils::HashSet};
use web_sys::*;

pub fn create_input_sources(
	mut commands: Commands,
	frame: NonSend<XrFrame>,
	reference_space: NonSend<XrReferenceSpace>,
	query: Query<(Entity, &mut Transform, &bevy_utils::BevyXrInputSource)>,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<bevy_utils::UnlitMaterial>>,
	input_source_asset_lookup: Res<bevy_utils::InputSourceAssetLookup>,
	input_source_lookup: NonSend<bevy_utils::InputSourceLookup>,
) {
	let spawned_sources = query
		.iter()
		.map(|(_, _, source)| source.hash)
		.collect::<HashSet<_>>();

	for (hash, input_source) in input_source_lookup.iter() {
		if spawned_sources.contains(hash) {
			continue;
		}
		//wait for asset path to be loaded
		let asset_path = match input_source_asset_lookup.get(hash) {
			Some(value) => value,
			None => continue,
		};
		// log!("creating input source for hash: {hash}");
		let _handle: Handle<Scene> = asset_server.load(asset_path);
		// let mut entity = commands.spawn((
		// 	SceneBundle {
		// 		scene: handle,
		// 		transform: BevyXrInputSource::grip_pose(
		// 			input_source,
		// 			&frame,
		// 			&reference_space,
		// 		)
		// 		.into(),
		// 		..default()
		// 	},
		// 	BevyXrInputSource::new(input_source),
		// ));
		let mut entity = commands.spawn((
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
				transform: BevyXrInputSource::grip_pose(
					input_source,
					&frame,
					&reference_space,
				)
				.into(),
				..default()
			},
			BevyXrInputSource::new(input_source),
		));
		bevy_utils::insert_input_handedness(
			&mut entity,
			input_source.handedness(),
		);
	}
}
