use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub type FnClosureUnit = Closure<dyn FnMut()>;
pub type FnClosure<T> = Closure<dyn FnMut(T)>;
pub type FnClosure2<T1, T2> = Closure<dyn FnMut(T1, T2)>;

pub fn set_panic_hook() { console_error_panic_hook::set_once(); }

pub fn performance_now() -> f64 {
	window().unwrap().performance().unwrap().now()
}
pub fn document_body() -> HtmlElement {
	let window = window().unwrap();
	window.document().unwrap().body().unwrap()
}
