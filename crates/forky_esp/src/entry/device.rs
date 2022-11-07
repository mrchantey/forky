#![no_std]
#![no_main]

use forky_esp::*;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
	let mut device = ESPDevice::new();

	loop {
		device.write("hello");
		device.write("world");
		device.delay(1000);
	}
}
