use std::collections::HashMap;

use crate::*;
use crate::*;
use bevy::prelude::*;
use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::*;
use wgpu::Extent3d;

#[rustfmt::skip]
pub fn update_xr_resources(world: &mut World) {
	let frame = world.non_send_resource::<XrFrame>();
	let session = frame.session();
	let gl_layer = frame.session().render_state().base_layer().unwrap();

	let reference_space = world.non_send_resource::<XrReferenceSpace>();

	//never None?
	let pose = frame.get_viewer_pose(&reference_space).unwrap();

	let views = pose.views().to_vec_typed();

	let bevy_views = views
		.iter()
		.map(|view| bevy_utils::BevyXrView::new(view, &gl_layer))
		.collect::<Vec<_>>();

	let input_sources: JsValue = session.input_sources().into();
	let input_sources: Array = input_sources.into();

	//TODO dont recreate hashmap each frame
	let bevy_input_sources = input_sources
    .iter()
    .fold(HashMap::new(), |mut acc, source| {
        let source = bevy_utils::BevyXrInputSource::new(source.into(), &frame, &reference_space);
        acc.insert(source.hash, source);
        acc
    });

	world.insert_non_send_resource(gl_layer);
	world.insert_resource(bevy_utils::BevyXrViewLookup(bevy_views));
	world.insert_resource(bevy_utils::BevyInputSourceLookup(bevy_input_sources));
}
