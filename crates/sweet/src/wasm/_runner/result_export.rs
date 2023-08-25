use js_sys::Reflect;
use wasm_bindgen::JsValue;
use web_sys::window;

use crate::SuiteResult;
use crate::TestRunnerResult;


pub struct ResultExport {
	// pub logs: Array,
}

const PROP_NAME_SUITE: &str = "_sweet_result_suite";
const PROP_NAME_TOTAL: &str = "_sweet_result_total";


impl ResultExport {
	pub fn init() {
		let window = window().unwrap();
		Reflect::set(&window, &PROP_NAME_SUITE.into(), &JsValue::UNDEFINED).unwrap();
		Reflect::set(&window, &PROP_NAME_TOTAL.into(), &JsValue::UNDEFINED).unwrap();
	}

	pub fn set_suites(suites:&Vec<SuiteResult>){
		let suites = serde_json::to_string(suites).unwrap();
		Reflect::set(&window().unwrap(), &PROP_NAME_SUITE.into(), &suites.into()).unwrap();
	}

	pub fn set_total(result:&TestRunnerResult){
		let result = serde_json::to_string(result).unwrap();
		Reflect::set(&window().unwrap(), &PROP_NAME_SUITE.into(), &result.into()).unwrap();
	}
}