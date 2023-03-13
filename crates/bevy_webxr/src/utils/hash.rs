use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

use wasm_bindgen::JsValue;

pub fn calculate_hash<T: Hash>(value: &T) -> u64 {
	let mut hasher = DefaultHasher::new();
	value.hash(&mut hasher);
	hasher.finish()
}

pub fn hash_js_value<T: Into<JsValue> + Clone, T2: Hasher>(
	value: &T,
	mut hasher: &mut T2,
) {
	Into::<JsValue>::into(value.clone())
		.as_string()
		.unwrap()
		.hash(&mut hasher);
}

pub fn js_as_string<T: Into<JsValue> + Clone>(value: &T) -> String {
	Into::<JsValue>::into(value.clone()).as_string().unwrap()
}
