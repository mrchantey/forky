use crate::*;
use anyhow::Result;
use js_sys::Array;
use js_sys::Function;
use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub struct TestCaseWasm {
	pub file: String,
	pub name: String,
	pub func: Function,
	pub config: TestCaseConfig,
}

impl TestCaseWasm {
	pub fn from_jsvalue(test: JsValue) -> Result<Self, JsValue> {
		let config = Reflect::get(&test, &"config".into())?.as_f64().unwrap();
		let config = TestCaseConfig::from_i32(config as i32);
		let name = Reflect::get(&test, &"name".into())?.as_string().unwrap();
		let file = Reflect::get(&test, &"file".into())?.as_string().unwrap();
		let func = Reflect::get(&test, &"func".into()).unwrap();
		let func: Function = func.into();
		Ok(Self {
			file,
			name,
			func,
			config,
		})
	}

	fn run_wasm(&self) -> Result<()> {
		let result = self.func.call0(&JsValue::NULL).unwrap();
		if result.is_string() {
			anyhow::bail!(result.as_string().unwrap())
		} else {
			Ok(())
		}
	}
}

impl TestCase for TestCaseWasm {
	fn file(&self) -> &str { self.file.as_str() }
	fn name(&self) -> &str { self.name.as_str() }
	fn config(&self) -> &TestCaseConfig { &self.config }
	fn run_func(&self) -> Result<()> { self.run_wasm() }
}
