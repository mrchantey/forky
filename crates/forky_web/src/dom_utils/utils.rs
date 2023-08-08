use wasm_bindgen::prelude::*;
use web_sys::*;

pub type FnClosureUnit = Closure<dyn FnMut()>;
pub type FnClosure<T> = Closure<dyn FnMut(T)>;
pub type FnClosure2<T1, T2> = Closure<dyn FnMut(T1, T2)>;

pub fn performance_now() -> f64 {
	window().unwrap().performance().unwrap().now()
}