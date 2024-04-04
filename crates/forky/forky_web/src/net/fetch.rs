use crate::*;
use anyhow::Result;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;


pub async fn fetch(url: &str) -> Result<Response> {
	let window = web_sys::window().unwrap();
	let res = JsFuture::from(window.fetch_with_str(url)).await.anyhow()?;
	let res = res.dyn_into().anyhow()?;
	Ok(res)
}
