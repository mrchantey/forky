use std::sync::{Arc, Mutex};

use crate::*;
use anyhow::Result;
use forky_wasm::*;


pub fn default_sketch() -> Result<SketchInstance> {
	let (leds) = default_peripherals()?;
	let instance = SketchInstance::from_default(&leds);
	Ok(instance)
}

pub fn default_peripherals() -> Result<(Arc<Mutex<dyn LedStrip + Send>>)> {
	let device = IDFDevice::new();
	let led_pin = device.peripherals.pins.gpio7.into_output().unwrap();
	let channel = device.peripherals.rmt.channel0;
	let leds = led_strip_rgbw!(led_pin, channel, 6)?.as_shared();

	Ok((leds))
}
