#[allow(unused_imports, unused_variables)]
use crate::*;
use esp32c3_hal::utils::{smartLedAdapter, SmartLedsAdapter};
use esp_hal_common::{
	pulse_control::{ConfiguredChannel, OutputChannel},
	utils::*,
	OutputPin,
};

use smart_leds::*;
pub type RGBA8 = RGBA<u8>;

#[macro_export]
macro_rules! led_controller_rgbw {
	($channel: expr, $pin: expr,$num_leds: literal) => {
		LEDControllerRGBW::<_, _, { $num_leds * 32 + 1 }, { $num_leds }>::new(
			$channel, $pin,
		)
	};
}

pub struct LEDControllerRGBW<
	CHANNEL,
	PIN,
	const BUFFER_SIZE: usize,
	const NUM_LEDS: usize,
> {
	// led: T,
	data: [RGBA8; NUM_LEDS],
	led: SmartLedsAdapterRGBW<CHANNEL, PIN, BUFFER_SIZE>,
}


impl<CC, PIN, const BUFFER_SIZE: usize, const NUM_LEDS: usize>
	LEDControllerRGBW<CC, PIN, BUFFER_SIZE, NUM_LEDS>
where
	CC: ConfiguredChannel,
	PIN: OutputPin,
{
	pub fn new<OC>(
		mut channel: OC,
		pin: PIN,
	) -> LEDControllerRGBW<CC, PIN, BUFFER_SIZE, NUM_LEDS>
	where
		OC: OutputChannel<CC>,
	{
		let mut led =
			SmartLedsAdapterRGBW::<CC, PIN, BUFFER_SIZE>::new(channel, pin);
		LEDControllerRGBW {
			led,
			data: [RGBA8 {
				r: 0,
				g: 0,
				b: 0,
				a: 0,
			}; NUM_LEDS],
		}
	}

	pub fn set_all(&mut self, r: u8, g: u8, b: u8, w: u8) {
		for i in 0..NUM_LEDS {
			self.data[i].r = r;
			self.data[i].g = g;
			self.data[i].b = b;
			self.data[i].a = w;
		}
	}

	pub fn set_white(&mut self, w: u8) {
		for i in 0..NUM_LEDS {
			self.data[i].a = w;
		}
	}

	pub fn set_hue(&mut self, hue: u8) {
		let color = hsv::Hsv {
			hue,
			sat: 255,
			val: 255,
		};
		for i in 0..NUM_LEDS {
			let RGB { r, g, b } = hsv::hsv2rgb(color);
			self.data[i].r = r;
			self.data[i].g = g;
			self.data[i].b = b;
		}
	}

	pub fn show(&mut self) {
		let data = self.data.iter();
		// let iter = brightness(gamma(), 10);
		let result = self.led.write(data.cloned()).unwrap();
		// match result{
		// 	Err(err)=>{
		// 		// err.
		// 	}
		// 	_=>{}
		// }
		// self.led.write().unwrap();
	}
}
