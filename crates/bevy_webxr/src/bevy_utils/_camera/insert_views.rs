use std::collections::HashMap;
use crate::*;
use bevy::prelude::*;
use derive_deref::{Deref, DerefMut};
use web_sys::*;

#[derive(Deref, DerefMut)]
pub struct ViewLookup(pub HashMap<u64, XrView>);

pub fn insert_views(world: &mut World) {
	let frame = world.non_send_resource::<XrFrame>();
	let gl_layer = world.non_send_resource::<XrWebGlLayer>();
	let reference_space = world.non_send_resource::<XrReferenceSpace>();

	let views = frame
		.get_viewer_pose(&reference_space)
		.unwrap()
		.views()
		.to_vec_typed()
		.iter()
		.map(|view| {
			(
				bevy_utils::BevyXrView::get_hash(view, &gl_layer),
				view.clone(),
			)
		})
		.collect::<HashMap<_, _>>();

	world.insert_non_send_resource(ViewLookup(views));
}
