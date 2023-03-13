use crate::{bevy_utils::BevyInputSourceLookup, *};
use bevy::{prelude::*, render::camera::Viewport};
use web_sys::*;

pub fn update_input_sources(
	// frame: NonSend<XrFrame>,
	input_sources: Res<BevyInputSourceLookup>,
	mut query: Query<(&mut Transform, &bevy_utils::InputSourceHash)>,
) {
	// let gl_layer = frame.session().render_state().base_layer().unwrap();

	for (mut transform, hash) in query.iter_mut() {
		let source = input_sources.get(hash).unwrap();
		transform.translation = source.grip_pose.position;
		transform.rotation = source.grip_pose.rotation;
	}
}
