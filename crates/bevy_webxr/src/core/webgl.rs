// use crate::{core::*, *};
use anyhow::Result;
// use js_sys::{Object, Promise, Reflect};
// use std::cell::RefCell;
// use std::rc::Rc;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::future_to_promise;
// use wasm_bindgen_futures::JsFuture;
use web_sys::*;


// pub fn get_webgl_context() -> Result<WebGl2RenderingContext, JsValue> {
// 	let document = web_sys::window().unwrap().document().unwrap();
// 	let canvas = document.get_element_by_id("canvas").unwrap();
// 	let canvas: web_sys::HtmlCanvasElement =
// 		canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

// 	let context = canvas
// 		.get_context("webgl2")? //get_context_with_context_options xrCompatible?
// 		.unwrap()
// 		.dyn_into::<WebGl2RenderingContext>()?;
// 	Ok(context)
// }
