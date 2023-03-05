use crate::*;
use anyhow::{Error, Result};
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;


pub fn run_xr_wgpu_test() {
	set_panic_hook();
	let _ = future_to_promise(run_xr_wgpu_test_async());
}

pub async fn run_xr_wgpu_test_async() -> Result<JsValue, JsValue> {
	//
	let gl = xr_utils::create_webgl_context(true).unwrap();
	// let mode = web_sys::XrSessionMode::Inline;
	let mode = web_sys::XrSessionMode::ImmersiveVr;
	let session = xr_utils::create_xr_session_with_mode(&gl, mode).await?;
	let xr_gl_layer = xr_utils::create_xr_gl_layer(&session, &gl).await?;
	let height = xr_gl_layer.framebuffer_height();
	let width = xr_gl_layer.framebuffer_width();
	let reference_space =
		xr_utils::get_reference_space(&session, &mode).await?;
	//
	let (device, queue) = wgpu_utils::init_wgpu().await.unwrap();

	xr_utils::run_xr_loop(&session, move |_time: f64, frame: XrFrame| {
		xr_utils::render_test_scene(&gl, &frame, &reference_space);
		wgpu_utils::render_xr_frame(&device, &queue, &frame);
	});


	Ok(JsValue::TRUE)
}
