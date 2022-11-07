//https://github.com/esp-rs/esp-hal/blob/main/esp32c3-hal/examples/hello_rgb.rs
#![no_std]
#![no_main]
use forky_esp::*;

#[entry]
fn main() -> ! {
	let mut device = ESPDevice::new();

	let mut controller =
		led_controller_rgb!(device.pulse.channel0, device.io.pins.gpio7, 4);

	loop {
		for hue in 0..=255 {
			controller.set_hue_all(hue);
			controller.show();
			device.timer.delay(10);
		}
		device.timer.delay(2000);
		device.logger.write("howdy!");
	}
}
