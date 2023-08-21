use js_sys::Array;
use js_sys::Reflect;
use wasm_bindgen::JsCast;
use web_sys::window;


pub struct LoggerExport {
	pub logs: Array,
}

const PROP_NAME: &str = "_sweet_log";

impl LoggerExport {
	pub fn init() -> Self {
		let window = window().unwrap();
		let logs = Array::new();
		Reflect::set(&window, &PROP_NAME.into(), &logs).unwrap();
		Self { logs }
	}
	pub fn get() -> Option<Self> {
		let window = window().unwrap();
		let logs = Reflect::get(&window, &PROP_NAME.into()).unwrap();
		if logs.is_undefined() {
			None
		} else {
			Some(Self {
				logs: logs.unchecked_into(),
			})
		}
	}
	pub fn get_or_init() -> Self { Self::get().unwrap_or_else(|| Self::init()) }

	pub fn push(&self, value: &str) {
		self.logs.push(&value.into());
	}
}
