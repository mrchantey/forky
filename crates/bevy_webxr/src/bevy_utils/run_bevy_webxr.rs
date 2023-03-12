// #![cfg(web_sys_unstable_apis)]
use crate::*;
use anyhow::Result;
use bevy::prelude::*;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::RenderApp;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;

pub fn run_bevy_webxr(app: App) {
	let _ = future_to_promise(run_bevy_webxr_async(app));
}
pub async fn run_bevy_webxr_async(mut app: App) -> Result<JsValue, JsValue> {
	set_panic_hook();
	app.add_plugin(bevy_utils::WebXrPlugin);
	let (session, _) = init_xr_render(&mut app).await?;
	let app = Arc::new(Mutex::new(app));
	xr_utils::run_xr_loop(&session, move |_time: f64, frame: XrFrame| {
		let mut app = app.lock().unwrap();
		app.insert_non_send_resource(frame);
		app.update();
	});
	Ok(JsValue::TRUE)
}

pub async fn init_xr_render(
	app: &mut App,
) -> Result<(XrSession, XrReferenceSpace), JsValue> {
	let gl = xr_utils::create_webgl_context(true)?;
	// xr_utils::clear_canvas(&gl)?;
	// let mode = web_sys::XrSessionMode::Inline;
	let mode = web_sys::XrSessionMode::ImmersiveVr;
	let session = xr_utils::create_xr_session_with_mode(&gl, mode).await?;

	let gl_layer = xr_utils::create_xr_gl_layer(&session, &gl).await?;
	let reference_space =
		xr_utils::get_reference_space(&session, &mode).await?;

	app.__()
		.insert_non_send_resource(mode)
		.insert_non_send_resource(session.clone())
		.insert_non_send_resource(reference_space.clone())
		.__();
	return Ok((session, reference_space));
}
