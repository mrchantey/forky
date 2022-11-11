use anyhow::Result;
use forky_idf as _;
use forky_wasm::*;
use std::{thread, time};


fn main() {
	// let mut _instance = build_simple()?;
	let _instance = build_simple_slowly().unwrap();
	println!("ok!");
	// Ok(())
	// loop {
	// println!("never yield!");
	// thread::sleep(time::Duration::from_millis(5))
	// thread::sleep(time::Duration::from_millis(2000))
	// }
}
