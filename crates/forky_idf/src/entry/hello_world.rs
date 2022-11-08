// use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
// esp_idf_sys::link_patches();
use std::{thread, time};
fn main() {
	let str = String::from("hello idf world!");
	loop {
		println!("{}", str);
		thread::sleep(time::Duration::from_secs(1))
	}
}
