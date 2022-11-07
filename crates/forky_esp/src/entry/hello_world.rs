//https://github.com/esp-rs/esp-hal/blob/main/esp32c3-hal/examples/hello_world.rs
#![no_std]
#![no_main]
use forky_esp::*;

#[entry]
fn main() -> ! {
	let mut device = ESPDevice::new();

	loop {
		device.write("hello");
		device.write("world");
		// device.delay(1000);
	}
}
