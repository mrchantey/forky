use crate::test_utils::render_test_scene;
// #![cfg(web_sys_unstable_apis)]
use crate::{core::*, *};
use anyhow::Result;
use bevy::prelude::*;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;
// use wgpu::util::DeviceExt;

#[derive(Resource)]
pub struct XrFrameRes {}

impl XrFrameRes {
	pub fn new(frame: XrFrame) -> XrFrameRes {
		// let viewer = frame.get_viewer_pose(XrReferenceSpace::)
		XrFrameRes {}
	}
}


pub fn run_bevy_webxr(app: App) {
	let _ = future_to_promise(run_bevy_webxr_async(app));
}
pub async fn run_bevy_webxr_async(app: App) -> Result<JsValue, JsValue> {
	set_panic_hook();

	let app = Arc::new(Mutex::new(app));
	let app = Arc::clone(&app);
	let app1 = app.clone();
	let app1 = app1.lock().unwrap();

	let device = app1.world.resource::<RenderDevice>().wgpu_device();
	let gl = create_webgl_context(true)?;

	let mode = web_sys::XrSessionMode::ImmersiveVr;
	let session = create_xr_session_with_mode(&gl, mode).await?;

	let gl_layer = create_xr_gl_layer(&session, &gl).await?;
	let width = gl_layer.framebuffer_width();
	let height = gl_layer.framebuffer_height();
	let framebuffer = gl_layer.framebuffer_unwrapped();
	// log!(
	// 	"got gl layer, width: {width}, height: {height}, framebuffer: {:?}",
	// 	gl_layer.framebuffer_unwrapped()
	// );
	let reference_space = get_reference_space(&session, &mode).await?;
	let reference_space = Arc::new(reference_space);
	let reference_space_2 = reference_space.clone();

	// let dst_texture = create_framebuffer_texture(&device, &gl_layer);

	// let gl_layer = session.render_state().base_layer().unwrap();
	// render_wgpu(&device, &opaque_texture, &gl_layer, mode).unwrap();


	//TODO
	//1. render camera to src texture texture
	//2. insert src/dest texture in render scene
	//3. add blit node


	run_xr_loop(&session, move |_time: f64, frame: XrFrame| {
		// render_test_scene(&gl, frame, reference_space_2.clone());
		// let app1 = app.lock().unwrap();
		// set_framebuffer(app1, &frame);
		let mut app = app.lock().unwrap();
		let device = app.world.resource::<RenderDevice>().wgpu_device();
		// render_wgpu(&device, &opaque_texture, &gl_layer, mode).unwrap();
		// let result = render_wgpu(&device, &opaque_texture, &frame, mode);
		// log!("frame: {:?}", result);
		app.update();
	});
	Ok(JsValue::from_str("success"))
}
