// #![cfg(web_sys_unstable_apis)]

use crate::{core::*, *};
use anyhow::Result;
use bevy::{prelude::*, winit::*};
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::{
	rc::Rc,
	sync::{Arc, Mutex},
	thread, time,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;




// pub fn run_bevy_webxr(app: App) {}
pub fn run_bevy_webxr(app: App) { future_to_promise(run_async(app)); }
// async fn run_async(app: App) -> Result<JsValue, JsValue> {
// 	set_panic_hook();
// 	// let app = Arc::new(Mutex::new(app));
// 	// let app_1 = Arc::clone(&app);
// 	// let request_animation_frame =
// 	// 	|session: &XrSession, f: &Closure<dyn FnMut(f64, XrFrame)>| -> u32 {
// 	// 		//TODO set camera pos etc
// 	// 		app_1.lock().unwrap().update();
// 	// 		session.request_animation_frame(f.as_ref().unchecked_ref())
// 	// 	};
// 	let gl = create_webgl_context(true).unwrap();
// 	let gl = Rc::new(gl);
// 	let session = Rc::new(RefCell::new(None));
// 	let result = JsFuture::from(init_webxr(session.clone(), gl)).await?;
// 	// log!("WebXR - Initialized");

// 	Ok(result)
// }
