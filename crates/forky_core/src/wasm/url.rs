use anyhow::Result;
use wasm_bindgen::JsValue;
use web_sys::{window, UrlSearchParams};

pub fn search_param(query: &str) -> Result<String, JsValue> {
	let search = window().unwrap().location().search()?;
	let params = UrlSearchParams::new_with_str(search.as_str())?;
	Ok(params.get(query).unwrap())
}
