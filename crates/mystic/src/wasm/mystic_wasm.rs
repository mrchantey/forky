use crate::{
	astro::{chart::Chart, planets::Y2000Day},
	tarot::spread::{self, TarotSpread},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MysticWasm {}

#[wasm_bindgen]
impl MysticWasm {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self { Self {} }

	pub fn hello_world(&self) -> String { String::from("hello world!") }

	pub fn tarot(&self) -> String {
		let spread = spread::PastPresentFuture::new();
		spread.to_string()
	}

	pub fn astro(&self, unix_ms: f64) -> String {
		let chart = Chart::new(Y2000Day::from_unix_ms(unix_ms as i64));
		chart.to_string()
	}
}
