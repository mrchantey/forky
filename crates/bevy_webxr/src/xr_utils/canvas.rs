use anyhow::Result;




use wasm_bindgen::prelude::*;

use web_sys::*;

pub const BEVY_CANVAS_ID: &str = "bevy_canvas";
pub const XR_CANVAS_ID: &str = "xr_canvas";
pub const BEVY_CANVAS_QUERY: &str = "canvas[data-bevy-webxr=\"bevy_canvas\"]";
pub const XR_CANVAS_QUERY: &str = "canvas[data-bevy-webxr=\"xr_canvas\"]";


//hack to get winit to render a small section
pub fn set_canvas_size() {
	let canvas = get_canvas(BEVY_CANVAS_QUERY).unwrap();
	canvas.set_attribute("width", "9999").unwrap();
	canvas.set_attribute("height", "5000").unwrap();
}

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

pub fn create_default_canvas() -> Result<HtmlCanvasElement, JsValue> {
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
