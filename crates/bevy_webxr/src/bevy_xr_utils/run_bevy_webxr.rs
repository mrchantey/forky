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
// use wgpu::util::DeviceExt;

// #[derive(Resource)]
// pub struct XrFrameRes {
// 	pub layer: &WebGlFramebuffer,
// }

// impl XrFrameRes {
// 	pub fn new(frame: XrFrame) -> XrFrameRes {
// 		// let viewer = frame.get_viewer_pose(XrReferenceSpace::)
// 		XrFrameRes {}
// 	}
// }


pub fn run_bevy_webxr(app: App) {
	let _ = future_to_promise(run_bevy_webxr_async(app));
}
pub async fn run_bevy_webxr_async(app: App) -> Result<JsValue, JsValue> {
	set_panic_hook();

	let app = Arc::new(Mutex::new(app));
	let app = Arc::clone(&app);
	let app1 = app.clone();
	let mut app1 = app1.lock().unwrap();

	let device = app1.world.resource::<RenderDevice>().wgpu_device();
	let gl = xr_utils::create_webgl_context(true)?;
	// xr_utils::clear_canvas(&gl)?;
	// let mode = web_sys::XrSessionMode::Inline;
	let mode = web_sys::XrSessionMode::ImmersiveVr;
	let session = xr_utils::create_xr_session_with_mode(&gl, mode).await?;

	let gl_layer = xr_utils::create_xr_gl_layer(&session, &gl).await?;
	// let width = gl_layer.framebuffer_width();
	// let height = gl_layer.framebuffer_height();
	// let framebuffer = gl_layer.framebuffer_unwrapped();
	// log!(
	// 	"got gl layer, width: {width}, height: {height}, framebuffer: {:?}",
	// 	gl_layer.framebuffer_unwrapped()
	// );
	let reference_space =
		xr_utils::get_reference_space(&session, &mode).await?;

	let blit_target = bevy_xr_utils::BlitTarget::new(&mut app1, &gl_layer);

	let mut render_app = app1.get_sub_app_mut(RenderApp).unwrap();
	render_app
		.insert_resource(blit_target)
		.init_resource::<bevy_utils::BlitPipeline>();

	// bevy_utils::insert_node(
	// 	render_app,
	// 	bevy_utils::ClearNode,
	// 	bevy_utils::CLEAR_PASS,
	// 	bevy_utils::END_MAIN_PASS,
	// 	bevy_utils::FINAL_PASS,
	// );
	bevy_utils::insert_node(
		render_app,
		bevy_utils::BlitNode,
		bevy_utils::BLIT_PASS,
		bevy_utils::END_MAIN_PASS,
		// bevy_utils::CLEAR_PASS,
		bevy_utils::FINAL_PASS,
	);

	xr_utils::run_xr_loop(&session, move |_time: f64, frame: XrFrame| {
		// xr_utils::render_test_scene(&gl, &frame, reference_space_2.clone());
		// set_framebuffer(app1, &frame);

		let mut app = app.lock().unwrap();
		let mut render_app = app.get_sub_app_mut(RenderApp).unwrap();
		// 	let dest_tex =
		// 		wgpu_utils::create_framebuffer_texture(device, &gl_layer);
		let device = render_app.world.resource::<RenderDevice>().wgpu_device();
		let gl_layer = frame.session().render_state().base_layer().unwrap();
		let dest_texture =
			wgpu_utils::create_framebuffer_texture(device, &gl_layer);
		let mut blit_target =
			render_app.world.resource_mut::<bevy_xr_utils::BlitTarget>();
		blit_target.update_dest(dest_texture);
		// 	// // blit_target.dest = dest_tex;
		//TODO update buffer
		app.update();
		// render_xr_frame(app, &frame);
	});
	Ok(JsValue::from_str("success"))
}
