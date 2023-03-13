use js_sys::Promise;
use wasm_bindgen::prelude::*;
use web_sys::*;


#[rustfmt::skip]
#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/@webxr-input-profiles/motion-controllers@1.0/dist/motion-controllers.module.js")]
extern "C" {
	#[wasm_bindgen(js_name = fetchProfile,catch)]
	// #[wasm_bindgen(catch)]
	pub async fn fetch_profile(input_source: &XrInputSource,base_path: &str) -> Result<JsValue, JsValue>;

	#[wasm_bindgen(js_name = MotionController)]
	type MotionController;

	// #[wasm_bindgen(method)]
	// fn some_method(&self);
}

