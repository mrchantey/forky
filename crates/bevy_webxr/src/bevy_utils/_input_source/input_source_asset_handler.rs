use crate::{bevy_utils::BevyXrInputSource, *};
use anyhow::Result;
use bevy::{prelude::*, utils::HashMap};

use std::sync::{Arc, Mutex};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local};
use web_sys::*;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct InputSourceAssetLookup(pub HashMap<u64, String>);


// #[derive(Default, Deref, DerefMut)]
// pub struct InputSourceAdded(pub u64);
// #[derive(Default, Deref, DerefMut)]
// pub struct InputSourceRemoved(pub u64);

const PROFILES_PATH:&str = "https://cdn.jsdelivr.net/npm/@webxr-input-profiles/assets@1.0/dist/profiles";

pub fn input_source_asset_loader(
	session: &XrSession,
	app: Arc<Mutex<App>>,
) -> Result<(), JsValue> {
	let f = Closure::<dyn FnMut(_)>::new(
		move |event: XrInputSourcesChangeEvent| {
			let added = event.added().to_vec_typed::<XrInputSource>();
			for input_source in added.iter() {
				insert_input_source(app.clone(), input_source.clone());
			}
			let removed = event.removed().to_vec_typed::<XrInputSource>();
			for input_source in removed.iter() {
				remove_input_source(app.clone(), input_source.clone());
			}
		},
	);
	session.add_event_listener_with_callback(
		"inputsourceschange",
		f.as_ref().unchecked_ref(),
	)?;
	f.forget();//terrible, memory leak
	Ok(())
}

fn remove_input_source(app: Arc<Mutex<App>>, input_source: XrInputSource) {
	let hash = BevyXrInputSource::get_hash(&input_source);
	let mut app = app.lock().unwrap();
	app.world
		.resource_mut::<InputSourceAssetLookup>()
		.remove(&hash);
}

fn insert_input_source(app: Arc<Mutex<App>>, input_source: XrInputSource) {
	spawn_local(async {
		insert_input_source_async(app, input_source).await.unwrap()
	});
}

// insert is async, could bug if removed before it finishes
async fn insert_input_source_async(
	app: Arc<Mutex<App>>,
	input_source: XrInputSource,
) -> Result<(), JsValue> {
	let result = bevy_utils::fetch_profile(&input_source, PROFILES_PATH)
		.await
		.unwrap();
	let mut asset_path = js_sys::Reflect::get(&result, &"assetPath".into())?
		.as_string()
		.unwrap();
	asset_path.push_str("#Scene0"); //bevy needs this

	let hash = BevyXrInputSource::get_hash(&input_source);

	let mut app = app.lock().unwrap();

	app.world
		.resource_mut::<InputSourceAssetLookup>()
		.insert(hash, asset_path);
	Ok(())
}
