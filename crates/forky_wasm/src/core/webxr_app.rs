use crate::{core::*, *};
use anyhow::Result;
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;


#[wasm_bindgen]
pub struct XrApp {
	session: Rc<RefCell<Option<XrSession>>>,
	gl: Rc<WebGl2RenderingContext>,
}

#[wasm_bindgen]
impl XrApp {
	#[wasm_bindgen(constructor)]
	pub fn new() -> XrApp {
		let session = Rc::new(RefCell::new(None));

		let xr_mode = true;
		let gl = Rc::new(create_webgl_context(xr_mode).unwrap());

		XrApp { session, gl }
	}

	pub fn init(&self) -> Promise {
		let session = self.session.clone();
		let gl = self.gl.clone();
		init_webxr(session, gl)
	}

	pub fn start(&self) {
		// run_xr(&self.session, move |_time: f64, frame: XrFrame| {
		// 	log!("frame");
		// });
	}
}
