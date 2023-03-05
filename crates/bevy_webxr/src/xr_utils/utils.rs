#![cfg(web_sys_unstable_apis)]
use crate::*;
use anyhow::{Error, Result};
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;


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
