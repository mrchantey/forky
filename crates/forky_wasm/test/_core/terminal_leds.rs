use std::sync::{Arc, Mutex};

use forky_wasm::{LedsInterface, SharedLeds};



// pub type SharedTerminalLeds = Arc<Mutex<TerminalLeds>>;

pub struct TerminalLeds {}

impl TerminalLeds {
	pub fn shared() -> SharedLeds {
		Arc::new(Mutex::new(TerminalLeds {}))
	}
}


impl LedsInterface for TerminalLeds {
	fn set_leds(&self, r: u8, g: u8, b: u8, w: u8) {
		println!("r: {}\tg: {}\tb: {}\tw: {}", r, g, b, w);
	}
}
