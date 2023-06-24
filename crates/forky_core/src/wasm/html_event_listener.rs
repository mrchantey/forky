use crate::log;

use super::*;
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast};
use web_sys::window;

pub struct HtmlEventListener<T> {
	closure: FnClosure<T>,
	name: &'static str,
}

impl<T> HtmlEventListener<T> {
	pub fn new<F>(name: &'static str, f: F) -> Self
	where
		F: FnMut(T) + Send + 'static,
		T: FromWasmAbi + 'static,
	{
		let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut(_)>);
		window()
			.unwrap()
			.add_event_listener_with_callback(
				name,
				closure.as_ref().unchecked_ref(),
			)
			.unwrap();
		Self { closure, name }
	}
}

impl<T> Drop for HtmlEventListener<T> {
	fn drop(&mut self) {
		log!("dropped event");
		window()
			.unwrap()
			.remove_event_listener_with_callback(
				self.name,
				self.closure.as_ref().unchecked_ref(),
			)
			.unwrap();
	}
}
