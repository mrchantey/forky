#![feature(imported_main)]
use crate::TestCaseConfig;
use crate::TestCaseDesc;
// use anyhow::Result;
use js_sys::Array;
use js_sys::Function;
use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::window;


fn to_test_case(test: JsValue) -> Result<TestCaseDesc, JsValue> {
	let config = Reflect::get(&test, &"config".into())?.as_f64().unwrap();
	let config = TestCaseConfig::from_i32(config as i32);
	let name = Reflect::get(&test, &"name".into())?.as_string().unwrap();
	let file = Reflect::get(&test, &"file".into())?.as_string().unwrap();
	let func = Reflect::get(&test, &"func".into()).unwrap();
	let func: Function = func.into();

	let func = move || -> anyhow::Result<()> {
		let result = func.call0(&JsValue::NULL).unwrap();
		if result.is_string() {
			anyhow::bail!(result.as_string().unwrap())
		} else {
			Ok(())
		}
	}; 
	Ok(TestCaseDesc {
		file,
		name,
		func,
		config,
	})
}


pub fn collect_tests() {
	let window = window().unwrap();
	let instance = Reflect::get(&window, &"_sweet_wasm".into()).unwrap();
	let tests = Object::entries(&instance.into());
	let tests = tests
		.iter()
		.map(|item| entry_func_tuple(item))
		.filter(|(key, _)| key.starts_with("_sweet_"))
		.map(|(_, func)| func.call0(&JsValue::NULL).unwrap());

	for test in tests {}
}

fn entry_func_tuple(item: JsValue) -> (String, Function) {
	let kvp: Array = item.into();
	let name = kvp.get(0).as_string().unwrap();
	let func: Function = kvp.get(1).into();
	(name, func)
}
