use core::fmt::Write;
use esp32c3_hal::Delay;
use esp_hal_common::{clock::Clocks, prelude::*};


pub struct Timer {
	pub delay: Delay,
}


impl Timer {
	pub fn new(clocks: &Clocks) -> Timer {
		let delay = Delay::new(&clocks);
		Timer { delay }
	}

	pub fn delay(&mut self, val: u16) { self.delay.delay_ms(val); }
}
