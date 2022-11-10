use crate::*;
use std::sync::{Arc, Mutex};


pub type SharedLeds = Arc<Mutex<dyn LedStrip + Send>>;

pub trait LedStrip {
	fn set_leds(&self, r: u8, g: u8, b: u8, w: u8);
}

pub struct Led;
impl Led {
	pub fn append_set_rgbw(builder: &mut SketchBuilder, leds: &SharedLeds) {
		// leds: &Arc<Mutex<dyn LedsInterface + Send>>,

		let leds = Arc::clone(&leds);
		builder.add_import_4(
			"led",
			"set_rgbw",
			move |caller, r: u32, g: u32, b: u32, w: u32| {
				leds.lock()
					.unwrap()
					.set_leds(r as u8, g as u8, b as u8, w as u8);
			},
		);
	}
}
