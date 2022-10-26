#![allow(dead_code, unused_imports, unused_variables)]
// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
use forky_fs::auto_mod::{log_changes, run_auto_mod};

fn main() {
	run_auto_mod();
	// for path in paths {
	// 	println!("Name: {}", path.unwrap().path().display())
	// }
	// log_changes("crates/forky", || {
	// 	println!("something happened!");
	// });
}
