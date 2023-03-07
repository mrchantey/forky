#![cfg(web_sys_unstable_apis)]
use crate::*;
use anyhow::{Error, Result};
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;


pub const BEVY_CANVAS_ID: &str = "bevy_canvas";
pub const XR_CANVAS_ID: &str = "xr_canvas";
pub const BEVY_CANVAS_QUERY: &str = "canvas[data-bevy-webxr=\"bevy_canvas\"]";
pub const XR_CANVAS_QUERY: &str = "canvas[data-bevy-webxr=\"xr_canvas\"]";

pub fn create_canvas(id: &'static str) -> Result<HtmlCanvasElement, JsValue> {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();

	let el = document.create_element("canvas")?;
	let canvas = el.dyn_into::<HtmlCanvasElement>()?;
	canvas.set_id(id);
	canvas.set_attribute("data-bevy-webxr", id).unwrap();
	document.body().unwrap().append_child(&canvas)?;
	Ok(canvas)
}

pub fn create_both_canvases() -> Result<HtmlCanvasElement, JsValue> {
	// create_canvas(BEVY_CANVAS_ID).unwrap();
	create_canvas(BEVY_CANVAS_ID)
	// create_canvas(XR_CANVAS_ID)
}


pub fn get_canvas(query: &'static str) -> Result<HtmlCanvasElement, JsValue> {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let canvas = document
		.query_selector(query)
		.unwrap()
		.expect("bevy_webxr - could not find canvas");
	let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
	Ok(canvas)
}


// pub fn clear_canvas(gl: &WebGl2RenderingContext) -> Result<JsValue, JsValue> {
// 	let canvas = get_canvas(XR_CANVAS_QUERY)?;
// 	// gl.base_l
// 	// gl.bind_framebuffer(target, framebuffer)

// 	Ok(JsValue::NULL)
// }

pub fn create_webgl_context(
	xr_mode: bool,
) -> Result<WebGl2RenderingContext, JsValue> {
	// let canvas = create_canvas()?;
	let canvas = get_canvas(BEVY_CANVAS_QUERY)?;
	// let canvas = get_canvas(XR_CANVAS_QUERY)?;

	let gl: WebGl2RenderingContext = if xr_mode {
		let gl_attribs = Object::new();
		Reflect::set(
			&gl_attribs,
			&JsValue::from_str("xrCompatible"),
			&JsValue::TRUE,
		)?;
		// Reflect::set(
		// 	&gl_attribs,
		// 	&JsValue::from_str("webgl2"),
		// 	&JsValue::TRUE,
		// )?;

		canvas
			.get_context_with_context_options("webgl2", &gl_attribs)?
			.unwrap()
			.dyn_into()?
	} else {
		canvas.get_context("webgl2")?.unwrap().dyn_into()?
	};

	Ok(gl)
}
