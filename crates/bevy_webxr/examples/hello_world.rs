use bevy_webxr::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

// fn main() {}

fn main() -> Result<(), JsValue> {
	log!("starting wasm bindgen...");
	let window = window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	let val = document.create_element("p")?;
	val.set_inner_html("Hello from Russy doodoo!");

	body.append_child(&val)?;

	log!("wasm bindgen initialized!");

	Ok(())
}
