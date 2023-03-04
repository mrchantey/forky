#![cfg(web_sys_unstable_apis)]
use crate::{core::*, *};
use anyhow::{Error, Result};
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;



pub fn create_webgl_context(
	xr_mode: bool,
) -> Result<WebGl2RenderingContext, JsValue> {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let body = document.body().unwrap();

	let el = document.create_element("canvas")?;
	el.set_id("canvas");
	body.append_child(&el)?;
	let canvas = el.dyn_into::<HtmlCanvasElement>()?;

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

	Ok(gl)
}

pub async fn create_xr_session(
	gl: &WebGl2RenderingContext,
) -> Result<XrSession, JsValue> {
	create_xr_session_with_mode(gl, XrSessionMode::Inline).await
}

pub async fn create_xr_session_with_mode(
	gl: &WebGl2RenderingContext,
	session_mode: XrSessionMode,
) -> Result<XrSession, JsValue> {
	let navigator: web_sys::Navigator = web_sys::window().unwrap().navigator();
	let xr = navigator.xr();
	let supports_session =
		JsFuture::from(xr.is_session_supported(session_mode)).await?;
	if supports_session == false {
		return Err(JsValue::from_str("XR session not supported"));
	}

	let session = JsFuture::from(xr.request_session(session_mode))
		.await?
		.into();

	let xr_gl_layer =
		XrWebGlLayer::new_with_web_gl2_rendering_context(&session, &gl)?;
	let mut render_state_init = XrRenderStateInit::new();
	render_state_init.base_layer(Some(&xr_gl_layer));
	session.update_render_state_with_state(&render_state_init);


	Ok(session)
}

pub async fn create_xr_gl_layer(
	session: &XrSession,
	gl: &WebGl2RenderingContext,
) -> Result<XrWebGlLayer, JsValue> {
	let layer = web_sys::XrWebGlLayer::new_with_web_gl2_rendering_context(
		&session, &gl,
	)?;
	let mut render_state_init = web_sys::XrRenderStateInit::new();
	render_state_init.base_layer(Some(&layer));
	session.update_render_state_with_state(&render_state_init);
	Ok(layer)
}


pub async fn get_reference_space(
	session: &XrSession,
	mode: &XrSessionMode,
) -> Result<XrReferenceSpace, JsValue> {
	let space_type = match mode {
		XrSessionMode::Inline => XrReferenceSpaceType::Viewer,
		XrSessionMode::ImmersiveVr => XrReferenceSpaceType::Local,
		XrSessionMode::ImmersiveAr => XrReferenceSpaceType::Local,
		_ => XrReferenceSpaceType::Viewer,
	};
	let reference_space =
		JsFuture::from(session.request_reference_space(space_type))
			.await?
			.into();
	Ok(reference_space)
}

pub fn run_xr_loop<F>(session: &XrSession, on_frame: F)
where
	F: Fn(f64, XrFrame) + 'static,
{
	let f = Rc::new(RefCell::new(None));
	let g = f.clone();
	*g.borrow_mut() = Some(Closure::new(move |_time: f64, frame: XrFrame| {
		on_frame(_time, frame.clone());
		let session: XrSession = frame.session();
		request_animation_frame_xr(&session, f.borrow().as_ref().unwrap());
	}));
	request_animation_frame_xr(&session, g.borrow().as_ref().unwrap());
}

fn request_animation_frame_xr(
	session: &XrSession,
	f: &Closure<dyn FnMut(f64, XrFrame)>,
) -> u32 {
	session.request_animation_frame(f.as_ref().unchecked_ref())
}


pub fn init_and_run_xr<F>(f: F)
where
	F: Fn(f64, XrFrame) + 'static,
{
	let _ = future_to_promise(init_and_run_xr_async(f));
}

pub async fn init_and_run_xr_async<F>(f: F) -> Result<JsValue, JsValue>
where
	F: Fn(f64, XrFrame) + 'static,
{
	let gl = create_webgl_context(true).unwrap();
	let session = create_xr_session(&gl).await.unwrap();
	// log!("WebXR - {}",result.as_string().unwrap());
	run_xr_loop(&session, f);
	Ok(JsValue::from_str("success"))
}





// pub fn init_webxr(
// 	session: Rc<RefCell<Option<XrSession>>>,
// 	gl: Rc<WebGl2RenderingContext>,
// ) -> Promise {
// 	let navigator: web_sys::Navigator = web_sys::window().unwrap().navigator();
// 	let xr = navigator.xr();
// 	// let session_mode = XrSessionMode::ImmersiveVr;
// 	let session_mode = XrSessionMode::Inline;


// 	future_to_promise(async move {
// 		let supports_session = JsFuture::from(xr.is_session_supported(session_mode)).await;
// 		let supports_session = supports_session.unwrap();
// 		if supports_session == false {
// 			log!("XR session not supported");
// 			return Ok(JsValue::from("XR session not supported"));
// 		}

// 		let xr_session = JsFuture::from(xr.request_session(session_mode)).await;
// 		let xr_session: XrSession = xr_session.unwrap().into();

// 		let xr_gl_layer =
// 			XrWebGlLayer::new_with_web_gl2_rendering_context(&xr_session, &gl)?;
// 		let mut render_state_init = XrRenderStateInit::new();
// 		render_state_init.base_layer(Some(&xr_gl_layer));
// 		xr_session.update_render_state_with_state(&render_state_init);

// 		let mut session = session.borrow_mut();
// 		session.replace(xr_session);

// 		Ok(JsValue::from("XR session initialized"))
// 	})
// }

// pub fn init_xr() {
// 	let xr_gl_layer =
// 		web_sys::XrWebGlLayer::new_with_web_gl2_rendering_context(
// 			&xr_session,
// 			&webgl2_context,
// 		)
// 		.unwrap();

// 	let framebuffer_height = xr_gl_layer.framebuffer_height();
// 	let framebuffer_width = xr_gl_layer.framebuffer_width();

// 	let mut render_state_init = web_sys::XrRenderStateInit::new();
// 	render_state_init.base_layer(Some(&xr_gl_layer));
// 	xr_session.update_render_state_with_state(&render_state_init);
// 	let reference_space: web_sys::XrReferenceSpace =
// 		wasm_bindgen_futures::JsFuture::from(
// 			xr_session.request_reference_space(reference_space_type),
// 		)
// 		.await
// 		.unwrap()
// 		.into();
// }
