use anyhow::{self};
use forky_idf::*;
use std::thread;
use std::time::Duration;
// const num_leds: i32 = 5;

#[rustfmt::skip]
fn main() -> anyhow::Result<()> {
	// esp_idf_sys::link_patches();
	let device = IDFDevice::new();
	let led_pin = device.peripherals.pins.gpio7.into_output().unwrap();
	let channel = device.peripherals.rmt.channel0;
	let mut strip = led_strip_rgbw!(led_pin,channel,6)?; 
	
	strip.set_all(8, 0, 0, 4);
	strip.show()?;
	loop {
		
		for i in 0..255{			
			// strip.set_all(0,i,0,4);
			strip.show()?;
			utility::sleep_ms(50);
		}
		utility::sleep_ms(1000);
	}
}
