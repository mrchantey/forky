// use web_sys::*;
use console_error_panic_hook;

pub fn set_panic_hook() { console_error_panic_hook::set_once(); }


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
	( $( $t:tt )* ) => {
			web_sys::console::log_1(&format!( $( $t )* ).into());
	}
}
