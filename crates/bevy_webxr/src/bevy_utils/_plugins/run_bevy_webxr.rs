// #![cfg(web_sys_unstable_apis)]
use crate::*;
use anyhow::Result;
use bevy::prelude::*;





use std::sync::{Arc, Mutex};

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::*;

// use super::{add_input_source_event, WebXrPlugin};

pub fn run_bevy_webxr(app: App) {
	spawn_local(async { run_bevy_webxr_async(app).await.unwrap() });
}
pub async fn run_bevy_webxr_async(mut app: App) -> Result<(), JsValue> {
	set_panic_hook();
	let (session, _) = init_xr_render(&mut app).await?;
	app.finish();
	let app = Arc::new(Mutex::new(app));
	bevy_utils::input_source_asset_loader(&session, app.clone())?;
	xr_utils::run_xr_loop(&session, move |_time: f64, frame: XrFrame| {
		let mut app = app.lock().unwrap();
		app.insert_non_send_resource(frame);
		app.update();
	});
	Ok(())
}

pub async fn init_xr_render(
	app: &mut App,
) -> Result<(XrSession, XrReferenceSpace), JsValue> {
	let gl = xr_utils::create_webgl_context(true)?;
	let mode = app.world.non_send_resource::<web_sys::XrSessionMode>();
	let reference_space_type = app
		.world
		.non_send_resource::<web_sys::XrReferenceSpaceType>();
	let session = xr_utils::create_xr_session_with_mode(&gl, *mode).await?;

	let _gl_layer = xr_utils::create_xr_gl_layer(&session, &gl)?;
	let reference_space =
		xr_utils::get_reference_space(&session, reference_space_type).await?;

	app.__()
		.insert_non_send_resource(session.clone())
		.insert_non_send_resource(reference_space.clone())
		.__();
	return Ok((session, reference_space));
}
