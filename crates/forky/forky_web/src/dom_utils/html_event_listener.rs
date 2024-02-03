use super::*;
use crate::*;
use forky_core::*;
use js_sys::Promise;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys::EventTarget;

pub struct HtmlEventListener<T> {
	pub closure: FnClosure<T>,
	pub target: EventTarget,
	pub name: &'static str,
}

impl<T> HtmlEventListener<T> {
	pub fn new<F>(name: &'static str, f: F) -> Self
	where
		F: FnMut(T) + 'static,
		T: FromWasmAbi + 'static,
	{
		Self::new_with_target(name, f, window().unwrap().unchecked_into())
	}
	pub fn new_with_target<F>(
		name: &'static str,
		f: F,
		target: EventTarget,
	) -> Self
	where
		F: FnMut(T) + 'static,
		T: FromWasmAbi + 'static,
	{
		let closure = Closure::from_func(f);
		// let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut(_)>);
		target
			.add_event_listener_with_callback(
				name,
				closure.as_ref().unchecked_ref(),
			)
			.unwrap();
		Self {
			target,
			name,
			closure,
		}
	}
	pub fn forget(self) { std::mem::forget(self); }
}

impl<T> Drop for HtmlEventListener<T> {
	fn drop(&mut self) {
		self.target
			.remove_event_listener_with_callback(
				self.name,
				self.closure.as_ref().unchecked_ref(),
			)
			.unwrap();
	}
}

impl HtmlEventListener<JsValue> {
	pub async fn wait(name: &'static str) -> JsValue {
		Self::wait_with_target(name, window().unwrap().unchecked_into()).await
	}
	pub async fn wait_with_target(
		name: &'static str,
		target: EventTarget,
	) -> JsValue {
		let listener: RcCell<Option<HtmlEventListener<JsValue>>> = rccell(None);

		let listener2 = listener.clone();
		let promise = Promise::new(&mut move |resolve, _reject| {
			let target = target.clone();
			*listener2.borrow_mut() =
				Some(HtmlEventListener::<JsValue>::new_with_target(
					name,
					move |value| {
						resolve.call1(&JsValue::NULL, &value).unwrap();
					},
					target,
				));
		});
		JsFuture::from(promise).await.unwrap()
	}
	// pub async fn wait_with_target_and_while_listening(
	// 	name: &'static str,
	// 	target: EventTarget,
	// 	mut while_listening: impl FnMut() + 'static,
	// ) -> JsValue {
	// 	let listener: RcCell<Option<HtmlEventListener<JsValue>>> = rccell(None);

	// 	let listener2 = listener.clone();
	// 	let promise = Promise::new(&mut move |resolve, _reject| {
	// 		let target = target.clone();
	// 		*listener2.borrow_mut() =
	// 			Some(HtmlEventListener::<JsValue>::new_with_target(
	// 				name,
	// 				move |value| {
	// 					resolve.call1(&JsValue::NULL, &value).unwrap();
	// 				},
	// 				target,
	// 			));
	// 		while_listening();
	// 	});
	// 	let result = JsFuture::from(promise).await.unwrap();
	// 	drop(listener);
	// 	result
	// }
}
