#![cfg(web_sys_unstable_apis)]
use crate::{core::*, *};
use anyhow::Result;
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;



pub fn create_webgl_context(
	xr_mode: bool,
) -> Result<WebGl2RenderingContext, JsValue> {
	let window = web_sys::window().expect("");
	let document = window.document().expect("");
	let body = document.body().expect("");

	let el = document.create_element("canvas")?;
	body.append_child(&el)?;
	let canvas = el.dyn_into::<HtmlCanvasElement>()?;

	let gl: WebGl2RenderingContext = if xr_mode {
		let gl_attribs = Object::new();
		Reflect::set(
			&gl_attribs,
			&JsValue::from_str("xrCompatible"),
			&JsValue::TRUE,
		)
		.unwrap();

		canvas
			.get_context_with_context_options("webgl2", &gl_attribs)?
			.unwrap()
			.dyn_into()?
	} else {
		canvas.get_context("webgl2")?.unwrap().dyn_into()?
	};

	Ok(gl)
}

pub fn init_webxr(
	session: Rc<RefCell<Option<XrSession>>>,
	gl: Rc<WebGl2RenderingContext>,
) -> Promise {
	let navigator: web_sys::Navigator = web_sys::window().unwrap().navigator();
	let xr = navigator.xr();
	let session_mode = XrSessionMode::ImmersiveVr;
	// let session_mode = XrSessionMode::Inline;

	let future_ = async move {
		let supports_session = JsFuture::from(xr.is_session_supported(session_mode)).await;
		let supports_session = supports_session.unwrap();
		if supports_session == false {
			log!("XR session not supported");
			return Ok(JsValue::from("XR session not supported"));
		}

		let xr_session = JsFuture::from(xr.request_session(session_mode)).await;
		let xr_session: XrSession = xr_session.unwrap().into();

		let xr_gl_layer =
			XrWebGlLayer::new_with_web_gl2_rendering_context(&xr_session, &gl)?;
		let mut render_state_init = XrRenderStateInit::new();
		render_state_init.base_layer(Some(&xr_gl_layer));
		xr_session.update_render_state_with_state(&render_state_init);

		let mut session = session.borrow_mut();
		session.replace(xr_session);

		Ok(JsValue::from("XR session initialized"))
	};

	future_to_promise(future_)
}

pub fn run_xr<F>(session: &Rc<RefCell<Option<XrSession>>>, on_frame: F)
where
	F: Fn(f64, XrFrame) + 'static,
{
	let f = Rc::new(RefCell::new(None));
	let g = f.clone();
	// let mut i = 0;
	*g.borrow_mut() = Some(Closure::new(move |_time: f64, frame: XrFrame| {
		on_frame(_time, frame.clone());
		// if i > 2 {
		// 	log!("All done!");
		// 	let _ = f.borrow_mut().take();
		// 	return;
		// }
		// i += 1;
		let sess: XrSession = frame.session();
		request_animation_frame_xr(&sess, f.borrow().as_ref().unwrap());
	}));

	let session: &Option<XrSession> = &session.borrow();
	let sess: &XrSession = if let Some(sess) = session {
		sess
	} else {
		return ();
	};
	request_animation_frame_xr(sess, g.borrow().as_ref().unwrap());
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
	let gl = Rc::new(gl);
	let session = Rc::new(RefCell::new(None));
	let result =
		JsFuture::from(init_webxr(session.clone(), gl.clone())).await?;
	// log!("WebXR - {}",result.as_string().unwrap());
	run_xr(&session, f);
	Ok(result)
}


pub fn run_xr_test() {
	log!("WebXR - Starting...");
	set_panic_hook();
	let _ = init_and_run_xr(move |_time: f64, _frame: XrFrame| {
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
