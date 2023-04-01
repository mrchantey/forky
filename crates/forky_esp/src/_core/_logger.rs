use core::fmt::Write;
use esp_hal_common::{pac::UART0, prelude::*, Serial};


pub struct Logger {
	pub serial: Serial<UART0>,
}


impl Logger {
	pub fn new(uart: UART0) -> Logger {
		let mut serial = Serial::new(uart);
		Logger { serial }
	}
	pub fn write(&mut self, val: &str) {
		writeln!(self.serial, "{}", val).unwrap();
	}
}
