use anyhow::Result;
use forky_core::wasm::*;
use forky_core::*;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{DeviceOrientationEvent, EventTarget, HtmlElement};

fn create_element(name:&str) -> Result<HtmlElement, JsValue> {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let el = document.create_element(name)?;
	let el = el.dyn_into::<HtmlElement>()?;
	document.body().unwrap().append_child(&el)?;
	Ok(el)
}

fn create_div() -> Result<HtmlElement, JsValue> {
	create_element("div")
}

fn run() -> Result<(), JsValue> {
	let window = web_sys::window().unwrap();
	let target: EventTarget = window.into();
	let el = create_div()?;
	el.set_inner_html("welcome to keera!");

	let el = create_div()?;
	el.set_inner_html("hello world!");

	let closure =
		Closure::wrap(Box::new(move |event: DeviceOrientationEvent| {
			let alpha = event.alpha().unwrap_or(0.0);
			let beta = event.beta().unwrap_or(0.0);
			let gamma = event.gamma().unwrap_or(0.0);

			let orientation =
				format!("Alpha: {}, Beta: {}, Gamma: {}", alpha, beta, gamma);
			log!("{orientation}");
			el.set_inner_html(orientation.as_str());
		}) as Box<dyn FnMut(_)>);

	target
		.add_event_listener_with_callback(
			"deviceorientation",
			closure.as_ref().unchecked_ref(),
		)
		.unwrap();
	closure.forget();

	Ok(())
}



fn main() {
	set_panic_hook();
	log!("hello world!");
	run().unwrap();
}
