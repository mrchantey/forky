use crate::*;
use bevy::{
	prelude::*,
};


use web_sys::*;

use super::BevyXrInputSource;

pub fn update_input_sources(
	mut commands: Commands,
	frame: NonSend<XrFrame>,
	reference_space: NonSend<XrReferenceSpace>,
	mut query: Query<(Entity, &mut Transform, &bevy_utils::BevyXrInputSource)>,
	input_sources: NonSend<bevy_utils::InputSourceLookup>,
) {
	for (entity, mut transform, source) in query.iter_mut() {
		match input_sources.get(&source.hash) {
			Some(input_source) => {
				let grip_pose = BevyXrInputSource::grip_pose(
					input_source,
					&frame,
					&reference_space,
				);
				transform.translation = grip_pose.position;
				transform.rotation = grip_pose.rotation;
			}
			None => {
				// log!("removing input source for hash: {}", source.hash);
				commands.entity(entity).despawn();
			}
		};
	}
}
