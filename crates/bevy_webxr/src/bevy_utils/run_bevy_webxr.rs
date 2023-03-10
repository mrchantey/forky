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
	let (session, reference_space) =
		bevy_utils::init_xr_render(&mut app).await?;
	let app = Arc::new(Mutex::new(app));
	xr_utils::run_xr_loop(&session, move |_time: f64, frame: XrFrame| {
		let mut app = app.lock().unwrap();
		update_xr(&mut app, &frame, &reference_space);
	});
	Ok(JsValue::TRUE)
}

pub fn update_xr(
	app: &mut App,
	frame: &XrFrame,
	reference_space: &XrReferenceSpace,
) {
	bevy_utils::update_framebuffer_texture(app, frame);
	bevy_utils::update_src_image_size(app, frame);
	bevy_utils::update_xr_tracking(app, frame, reference_space);
	app.update();
}
