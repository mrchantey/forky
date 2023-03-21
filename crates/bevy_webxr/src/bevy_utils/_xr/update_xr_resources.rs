use std::collections::HashMap;

use crate::*;
use crate::*;
use bevy::prelude::*;
use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::*;
use wgpu::Extent3d;

pub fn insert_gl_layer(world: &mut World) {
	let gl_layer = world
		.non_send_resource::<XrFrame>()
		.session()
		.render_state()
		.base_layer()
		.unwrap();
	world.insert_non_send_resource(gl_layer);
}
