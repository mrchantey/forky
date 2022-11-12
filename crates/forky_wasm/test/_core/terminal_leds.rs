use forky_wasm::*;
use std::sync::{Arc, Mutex};


// pub type SharedTerminalLeds = Arc<Mutex<TerminalLeds>>;

pub struct TerminalLeds {
	leds: Vec<rgb::RGBA8>,
}

impl TerminalLeds {
	pub fn new(size: usize) -> TerminalLeds {
		TerminalLeds {
			leds: vec![
				rgb::RGBA8 {
					r: 0,
					g: 0,
					b: 0,
					a: 0,
				};
				size
			],
		}
	}
}


impl LedStrip for TerminalLeds {
	fn set_leds(&mut self, r: u8, g: u8, b: u8, w: u8) {
		for mut led in &mut self.leds {
			led.r = r;
			led.g = g;
			led.b = b;
			led.a = w;
		}
	}
	fn show(&mut self) {
		for rgb::RGBA8 { r, g, b, a } in &self.leds {
			println!("r: {}\tg: {}\tb: {}\tw: {}", r, g, b, a);
		}
	}

	fn as_shared(self) -> SharedLeds { Arc::new(Mutex::new(self)) }
}
