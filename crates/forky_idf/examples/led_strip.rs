use anyhow::{self};
use forky_idf::*;
use std::thread;
use std::time::Duration;
// const num_leds: i32 = 5;

#[rustfmt::skip]
fn main() -> anyhow::Result<()> {
	// esp_idf_sys::link_patches();
	let device = IDFDevice::new();

	let led = device.peripherals.pins.gpio7.into_output().unwrap();
	let channel = device.peripherals.rmt.channel0;
	// device.peripherals.

	let mut strip = LedStrip::<6, 193>::new(led, channel)?;
	strip.set_all(8, 0, 0, 4);
	strip.show()?;
	loop {
		
		for i in 0..255{			
			// strip.set_all(0,i,0,4);
			strip.show()?;
			thread::sleep(Duration::from_millis(50));
		}
		thread::sleep(Duration::from_millis(1000));
	}
}
