use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, HtmlCanvasElement, HtmlElement};


pub fn create_element(name: &str) -> Result<Element, JsValue> {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let el = document.create_element(name)?;
	// let el = el.dyn_into::<HtmlElement>()?;
	document.body().unwrap().append_child(&el)?;
	Ok(el)
}

pub fn create_div() -> Result<HtmlElement, JsValue> {
	let el = create_element("div")?.dyn_into::<HtmlElement>()?;
	Ok(el)
}

pub fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
	let el = create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
	Ok(el)
}
