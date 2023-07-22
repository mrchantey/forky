use super::*;
use crate::*;
use forky_core::*;
use js_sys::Array;
use js_sys::Function;
use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub struct TestCollectorWasm(pub Vec<TestSuite<TestCaseWasm>>);

impl TestCollectorWasm {
	pub fn new() -> Self {
		let window = window().unwrap();
		let instance = Reflect::get(&window, &"_sweet_wasm".into()).unwrap();
		let tests = Object::entries(&instance.into());
		let tests = tests
			.iter()
			.map(|item| entry_func_tuple(item))
			.filter(|(key, _)| key.starts_with("_sweet_"))
			.map(|(_, func)| func.call0(&JsValue::NULL).unwrap())
			.map(|test| TestCaseWasm::from_jsvalue(test).unwrap())
			.collect::<Vec<_>>();

		for _test in tests {
			log!("{:?}", _test);
		}
		Self(Vec::new())
	}
}

pub fn collect_tests() {}

fn entry_func_tuple(item: JsValue) -> (String, Function) {
	let kvp: Array = item.into();
	let name = kvp.get(0).as_string().unwrap();
	let func: Function = kvp.get(1).into();
	(name, func)
}
