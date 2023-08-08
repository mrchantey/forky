use super::*;
use crate::*;
use anyhow::Result;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[derive(Debug, Clone)]
pub struct TestCollectorWasm(pub Vec<TestSuiteWasm>);

impl TestCollectorWasm {
	pub fn new() -> Self { Self(Self::collect_cases_to_suites()) }

	pub fn collect_case(id: usize) -> Result<TestCaseWasm> {
		let wasm =
			Reflect::get(&window().unwrap(), &"_sweet_wasm".into()).unwrap();

		if let Ok(case_func) =
			Reflect::get(&wasm, &format!("_sweet_{id}").into())
		{
			if case_func.is_function() {
				let case_func: Function = case_func.into();
				let case_val = case_func.call0(&JsValue::NULL).unwrap();
				let case = TestCaseWasm::from_jsvalue(case_val).unwrap();
				return Ok(case);
			}
		}
		anyhow::bail!("Test not found, id: {id}")
	}
}

impl TestCollector<TestCaseWasm, TestSuiteWasm> for TestCollectorWasm {
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
