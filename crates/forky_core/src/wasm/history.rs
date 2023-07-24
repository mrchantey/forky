use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Url;

pub struct History;

impl History {
	pub fn push(path: &str) {
		window()
			.unwrap()
			.history()
			.unwrap()
			.push_state_with_url(&JsValue::UNDEFINED, "", Some(path))
			.unwrap();
	}
	pub fn push_preserve_params(path: &str) {
		let location = window().unwrap().location();
		let href = location.href().unwrap();
		let url = Url::new(&href).unwrap();
		url.set_pathname(path);
		Self::push(url.href().as_str());
	}
	pub fn replace(path: &str) {
		window()
			.unwrap()
			.history()
			.unwrap()
			.replace_state_with_url(&JsValue::UNDEFINED, path, Some(path))
			.unwrap();
	}

	// pub fn replace_params(params:UrlSearchParams){


	// }
}
