use forky_core::wasm::*;
use forky_core::*;
use keera::debug::hello_plot;

fn main() {
	set_panic_hook();
	log!("hello world!");
	hello_plot().unwrap();
	// hello_orientation().unwrap();
}
