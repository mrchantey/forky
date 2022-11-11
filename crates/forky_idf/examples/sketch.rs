use std::sync::Arc;

use anyhow::Result;
use forky_idf::*;
use forky_wasm::*;


fn main() -> Result<()> {
	let device = IDFDevice::new();
	let led_pin = device.peripherals.pins.gpio7.into_output().unwrap();
	let channel = device.peripherals.rmt.channel0;
	let leds = led_strip_rgbw!(led_pin, channel, 6)?.as_shared();

	let mut instance = SketchInstance::from_default(&leds);
	instance.run();
	Arc::clone(&leds).lock().unwrap().show();
	println!("sketch ok!");
	Ok(())
}
