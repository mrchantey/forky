use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

pub fn forget<T>(val: T) {
	let closure = Closure::<dyn FnMut()>::new(move || {
		let _ = val;
	});
	closure.forget();
}
pub fn forget_func<F>(f: F)
where
	F: FnMut() + 'static,
{
	let closure = Closure::<dyn FnMut()>::new(f);
	closure.forget();
}

pub fn run_async<F>(fut: F)
where
	F: Future<Output = Result<JsValue, JsValue>> + 'static,
{
	let _ = future_to_promise(fut).catch(&Closure::new(|val| {
		// log::error!("Wasm Error: {:?}", val);
		web_sys::console::error_1(&val)
	}));
}
