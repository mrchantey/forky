use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::JsValue;



pub fn build_test_case(
	id: &JsValue,
	name: &JsValue,
	file: &JsValue,
	func: &JsValue,
	config: &JsValue,
) -> JsValue {
	let obj = Object::new();
	Reflect::set(&obj, &"id".into(), id).unwrap();
	Reflect::set(&obj, &"name".into(), name).unwrap();
	Reflect::set(&obj, &"file".into(), file).unwrap();
	Reflect::set(&obj, &"func".into(), func).unwrap();
	Reflect::set(&obj, &"config".into(), config).unwrap();
	obj.into()
}
