#![cfg(web_sys_unstable_apis)]
use crate::*;
use anyhow::{Result};
use js_sys::{Object, Reflect};



use wasm_bindgen::prelude::*;

use web_sys::*;


pub fn create_webgl_context(
	xr_mode: bool,
) -> Result<WebGl2RenderingContext, JsValue> {
	// let canvas = create_canvas()?;
	let canvas = xr_utils::get_canvas(xr_utils::BEVY_CANVAS_QUERY)?;
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

	// gl.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
	Ok(gl)
}
