#![feature(imported_main)]
use anyhow::Result;
use forky_core::*;
use js_sys::Array;
use js_sys::Function;
use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::window;


pub fn collect_tests() {
	let window = window().unwrap();
	let instance = Reflect::get(&window, &"_sweet_wasm".into()).unwrap();
	let tests = Object::entries(&instance.into());
	let tests = tests
		.iter()
		.map(|item| entry_func_tuple(item))
		.filter(|(key, _)| key.starts_with("_sweet_"))
		.map(|(_, func)| func.call0(&JsValue::NULL).unwrap());

	for test in tests {
		let val = Reflect::get(&test, &"func".into()).unwrap();
		let func: Function = val.into();
		let result = func.call0(&JsValue::NULL).unwrap();
		if result.is_string() {
			log!("{}", result.as_string().unwrap());
		} else {
			log!("OK!");
		}
	}
}

fn entry_func_tuple(item: JsValue) -> (String, Function) {
	let kvp: Array = item.into();
	let name = kvp.get(0).as_string().unwrap();
	let func: Function = kvp.get(1).into();
	(name, func)
}
