use crate::*;
use anyhow::{Error, Result};
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;



pub fn run_xr_test() {
	log!("WebXR - Starting...");
	set_panic_hook();
	let _ = xr_utils::init_and_run_xr(move |_time: f64, _frame: XrFrame| {
		log!("frame");
	});
	// let gl = create_webgl_context(true).unwrap();
	// let gl = Rc::new(gl);
	// let session = Rc::new(RefCell::new(None));
	// let result =
	// 	JsFuture::from(init_webxr(session.clone(), gl.clone())).await?;
	// run_xr(&session, );
	log!("WebXR - Initialized");
}
fn viewport_rect(view: &XrViewport) -> (i32, i32, i32, i32) {
	(view.x(), view.y(), view.width(), view.height())
}

pub fn render_test_scene(
	gl: &WebGl2RenderingContext,
	frame: &XrFrame,
	reference_space: &XrReferenceSpace,
	// reference_space: Arc<XrReferenceSpace>,
) {
	let gl_layer = frame.session().render_state().base_layer().unwrap();
	let framebuffer = gl_layer.framebuffer();
	gl.bind_framebuffer(
		WebGl2RenderingContext::FRAMEBUFFER,
		framebuffer.as_ref(),
	);
	
	let mut i = 0;
	let pose = frame.get_viewer_pose(&reference_space).unwrap();
	pose.views().iter().for_each(|view| {
		gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
		let (x, y, width, height) =
		viewport_rect(&gl_layer.get_viewport(&view.into()).unwrap().into());
		// log!("viewport: x:{x},y:{y},width:{width},height:{height}");
		// gl.viewport(x, y, width, height); //for vertices
		gl.scissor(x, y, width, height); //for clear
		if i == 0 {
			gl.clear_color(0.2, 0.0, 0.0, 1.0);
		} else {
			gl.clear_color(0.0, 0.2, 0.0, 1.0);
		}
		gl.clear(
			WebGl2RenderingContext::COLOR_BUFFER_BIT
			| WebGl2RenderingContext::DEPTH_BUFFER_BIT,
		);
		gl.disable(WebGl2RenderingContext::SCISSOR_TEST);
		i += 1;
	});
}
