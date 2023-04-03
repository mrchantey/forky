use crate::*;
use bevy::prelude::*;
use js_sys::Array;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::*;

#[derive(Deref, DerefMut)]
pub struct InputSourceLookup(pub HashMap<u64, XrInputSource>);

pub fn insert_input_sources(world: &mut World) {
	let session = world.non_send_resource::<XrSession>();

	let input_sources: JsValue = session.input_sources().into();
	let input_sources = Into::<Array>::into(input_sources)
		.iter()
		.map(|source| source.into())
		.map(|source| {
			(bevy_utils::BevyXrInputSource::get_hash(&source), source)
		})
		.collect::<HashMap<_, _>>();

	world.insert_non_send_resource(InputSourceLookup(input_sources));
}
