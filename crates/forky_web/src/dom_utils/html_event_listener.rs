use super::*;
use forky_core::*;
use js_sys::Promise;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys::Window;

pub struct HtmlEventListener<T> {
	closure: FnClosure<T>,
	name: &'static str,
}

impl<T> HtmlEventListener<T> {
	pub fn new<F>(name: &'static str, f: F) -> Self
	where
		F: FnMut(T) + 'static,
		T: FromWasmAbi + 'static,
	{
		Self::new_with_window(name, f, window().unwrap())
	}
	pub fn new_with_window<F>(name: &'static str, f: F, window: Window) -> Self
	where
		F: FnMut(T) + 'static,
		T: FromWasmAbi + 'static,
	{
		let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut(_)>);
		window
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
		window()
			.unwrap()
			.remove_event_listener_with_callback(
				self.name,
				self.closure.as_ref().unchecked_ref(),
			)
			.unwrap();
	}
}

impl HtmlEventListener<JsValue> {
	pub async fn wait(name: &'static str) -> JsValue {
		Self::wait_with_window(name, window().unwrap()).await
	}
	pub async fn wait_with_window(
		name: &'static str,
		window: Window,
	) -> JsValue {
		let listener: RcCell<Option<HtmlEventListener<JsValue>>> = rccell(None);

		let listener2 = listener.clone();
		let promise = Promise::new(&mut move |resolve, _reject| {
			let window = window.clone();
			*listener2.borrow_mut() =
				Some(HtmlEventListener::<JsValue>::new_with_window(
					name,
					move |value| {
						resolve.call1(&JsValue::NULL, &value).unwrap();
					},
					window,
				));
		});
		let result = JsFuture::from(promise).await.unwrap();
		drop(listener);
		result
	}
}
