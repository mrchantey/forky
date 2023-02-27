// #![cfg(web_sys_unstable_apis)]
use crate::{core::*, *};
use anyhow::Result;
use bevy::prelude::*;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::renderer::{RenderQueue, RenderDevice};
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

fn viewport_rect(view: &XrViewport) -> (i32, i32, i32, i32) {
	(view.x(), view.y(), view.width(), view.height())
}

pub fn render(frame: XrFrame, reference_space: Arc<XrReferenceSpace>) {
	let pose = frame.get_viewer_pose(&reference_space).unwrap();
	let gl = get_webgl_context().unwrap(); //slow i think
	let gl_layer = frame.session().render_state().base_layer().unwrap();
	let buff = gl_layer.framebuffer();

	gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, buff.as_ref());


	let mut i = 0;
	pose.views().iter().for_each(|view| {
		// log!("viewport: x:{},y:{},width:{},height:{}",viewport.x(),viewport.y(),viewport.width(),viewport.height());
		let (x, y, width, height) =
			viewport_rect(&gl_layer.get_viewport(&view.into()).unwrap().into());
		gl.viewport(x, y, width, height); //for vertices
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
		//TODO render app.world.resource::<Some_Render_Texture>()
		//gl_layer.get_viewport(&view.into()).unwrap().into()
		i += 1;
	});
	
}

pub fn run_bevy_webxr(app: App) {
	let _ = future_to_promise(run_bevy_webxr_async(app));
}
pub async fn run_bevy_webxr_async(app: App) -> Result<JsValue, JsValue> {
	set_panic_hook();

	let queue = app.world.resource::<RenderQueue>();
	// let device = app.world.resource::<RenderDevice>();
	// device.
// 	device.create_(&BufferInitDescriptor {
//     label: None,
//     contents: &[0u8; 1024 * 1024 * 4],
//     usage: BufferUsage::MAP_READ | BufferUsage::COPY_DST,
// });

	let app = Arc::new(Mutex::new(app));
	let app = Arc::clone(&app);
	let gl = create_webgl_context(true).unwrap();
	let gl = Rc::new(gl);
	gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
	let session = Rc::new(RefCell::new(None));

	let result =
		JsFuture::from(init_webxr(session.clone(), gl.clone())).await?;
	// log!("WebXR - {}",result.as_string().unwrap());
	let session_2 = session.clone().borrow().clone().unwrap();
	let reference_space: XrReferenceSpace = JsFuture::from(
		session_2.request_reference_space(XrReferenceSpaceType::Viewer),
		// session_2.request_reference_space(XrReferenceSpaceType::Local),
	)
	.await?
	.into();
	let reference_space = Arc::new(reference_space);
	let reference_space_2 = reference_space.clone();

	run_xr(&session, move |_time: f64, frame: XrFrame| {
		let app1 = app.lock().unwrap();
		set_framebuffer(app1, &frame);
		render(frame, reference_space_2.clone());
		let mut app = app.lock().unwrap();
		app.update();
	});
	Ok(result)
}
