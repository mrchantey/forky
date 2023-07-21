#![feature(imported_main)]
use forky_core::*;
pub use sweet::*;
use wasm_bindgen::prelude::*;

test! {"works", skip,
	log!("woah!");
	expect(true).to_be_true()?;
}
