use esp_idf_sys as _;
// esp_idf_sys::link_patches();
use std::{thread, time};
fn main() {
	let str = String::from("hello idf world!");
	loop {
		println!("{}", str);
		thread::sleep(time::Duration::from_secs(1))
	}
}
