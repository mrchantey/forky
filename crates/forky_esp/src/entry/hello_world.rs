//https://github.com/esp-rs/esp-hal/blob/main/esp32c3-hal/examples/hello_world.rs
#![no_std]
#![no_main]
use forky_esp::*;

#[entry]
fn main() -> ! {
	let mut device = ESPDevice::new();

	loop {
		device.logger.write("hello world");
		device.timer.delay(1000);
	}
}
