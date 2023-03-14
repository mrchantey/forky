use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

use wasm_bindgen::JsValue;
use web_sys::XrViewport;

pub fn calculate_hash<T: Hash>(value: &T) -> u64 {
	let mut hasher = DefaultHasher::new();
	value.hash(&mut hasher);
	hasher.finish()
}

pub fn hash_js_value<T1: Into<JsValue> + Clone, T2: Hasher>(
	value: &T1,
	mut hasher: &mut T2,
) {
	Into::<JsValue>::into(value.clone())
		.as_string()
		.unwrap()
		.hash(&mut hasher);
}

pub fn hash_vec<T1, T2>(value: &Vec<T1>, mut hasher: &mut T2)
where
	T1: Hash,
	T2: Hasher,
{
	for item in value.iter() {
		item.hash(&mut hasher);
	}
}


pub fn hash_vec_f32<T2>(value: &Vec<f32>, mut hasher: &mut T2)
where 
	T2: Hasher,
{
	for item in value.iter() {
		item.to_bits().hash(&mut hasher);
	}
}

pub fn hash_viewport<T2>(value: &XrViewport, mut hasher: &mut T2)
where 
	T2: Hasher,
{
	value.x().hash(&mut hasher);
	value.y().hash(&mut hasher);
	value.width().hash(&mut hasher);
	value.height().hash(&mut hasher);
}

pub fn js_as_string<T: Into<JsValue> + Clone>(value: &T) -> String {
	Into::<JsValue>::into(value.clone()).as_string().unwrap()
}
