use console_error_panic_hook;
use web_sys::*;

pub fn set_panic_hook() { console_error_panic_hook::set_once(); }



pub fn performance_now() -> f64 {
	window().unwrap().performance().unwrap().now()
}
