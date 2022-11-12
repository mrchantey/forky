use esp_idf_sys as _;
use forky_idf::utility;
// esp_idf_sys::link_patches();
use std::{thread, time};
fn main() {
	let str = String::from("hello browser!");
	loop {
		println!("{}", str);
		utility::sleep_ms(1000);
	}
}
