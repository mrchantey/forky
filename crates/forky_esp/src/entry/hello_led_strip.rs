//https://github.com/esp-rs/esp-hal/blob/main/esp32c3-hal/examples/hello_rgb.rs
#![no_std]
#![no_main]
use forky_esp::*;

#[entry]
fn main() -> ! {
	let mut device = ESPDevice::new();

	let mut controller =
		led_controller_rgbw!(device.pulse.channel0, device.io.pins.gpio7, 5);
	// led_controller_rgb!(device.pulse.channel0, device.io.pins.gpio7, 4);

	controller.set_white(127);
	controller.set_all(0, 0, 4, 4);
	controller.show();
	loop {
		for h in 0..=255 {
			// controller.set_hue(h);
			// controller.show();
			device.timer.delay(100);
		}
		// device.timer.delay(2000);
		// device.logger.write("howdy!");
	}
}
