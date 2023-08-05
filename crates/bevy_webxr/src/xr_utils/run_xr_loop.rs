use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::*;

pub fn init_and_run_xr<F>(f: F)
where
	F: Fn(f64, XrFrame) + 'static,
{
	let _ = future_to_promise(async {
		let gl = super::create_webgl_context(true)?;
		let session = super::create_xr_session(&gl).await?;
		// log!("WebXR - {}",result.as_string().unwrap());
		run_xr_loop(&session, f);
		Ok(JsValue::UNDEFINED)
	});
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


