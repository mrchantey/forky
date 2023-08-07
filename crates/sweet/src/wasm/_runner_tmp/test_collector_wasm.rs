use super::*;
use crate::*;
use js_sys::Array;
use js_sys::Function;
use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[derive(Debug, Clone)]
pub struct TestCollectorWasm(pub Vec<TestSuiteWasm>);

impl TestCollectorWasm {
	pub fn new() -> Self { Self(Self::collect_cases_to_suites()) }
}

impl TestCollector<TestCaseWasm, TestSuiteWasm>
	for TestCollectorWasm
{
	fn suites(&self) -> &Vec<TestSuiteWasm> { &self.0 }
	fn collect_cases() -> Vec<TestCaseWasm> {
		let window = window().unwrap();
		let instance = Reflect::get(&window, &"_sweet_wasm".into()).unwrap();
		let tests = Object::entries(&instance.into());
		tests
			.iter()
			.map(|item| entry_func_tuple(item))
			.filter(|(key, _)| key.starts_with("_sweet_"))
			.map(|(_, func)| func.call0(&JsValue::NULL).unwrap())
			.map(|test| TestCaseWasm::from_jsvalue(test).unwrap())
			.collect::<Vec<_>>()
	}
}

fn entry_func_tuple(item: JsValue) -> (String, Function) {
	let kvp: Array = item.into();
	let name = kvp.get(0).as_string().unwrap();
	let func: Function = kvp.get(1).into();
	(name, func)
}
