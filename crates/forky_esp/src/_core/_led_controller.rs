#[allow(unused_imports, unused_variables)]
use crate::ESPDevice;
use esp32c3_hal::utils::{smartLedAdapter, SmartLedsAdapter};
use esp_hal_common::{
	pulse_control::OutputChannel, utils::smart_leds_adapter::LedAdapterError,
	OutputPin,
};
use smart_leds::{
	brightness, gamma,
	hsv::{hsv2rgb, Hsv},
	SmartLedsWrite, RGB, RGB8,
};


#[macro_export]
macro_rules! led_controller_rgb {
	($channel: expr, $pin: expr,$num_leds: literal) => {
		LEDController::<_, _, { $num_leds * 24 + 1 }, { $num_leds }>::new(
			$channel, $pin,
		)
	};
}

pub struct LEDController<
	CHANNEL,
	PIN,
	const BUFFER_SIZE: usize,
	const NUM_LEDS: usize,
> {
	// led: T,
	data: [RGB<u8>; NUM_LEDS],
	led: SmartLedsAdapter<CHANNEL, PIN, BUFFER_SIZE>,
}

impl<CHANNEL, PIN, const BUFFER_SIZE: usize, const NUM_LEDS: usize>
	LEDController<CHANNEL, PIN, BUFFER_SIZE, NUM_LEDS>
where
	CHANNEL: OutputChannel,
	PIN: OutputPin,
{
	pub fn new(
		mut channel: CHANNEL,
		pin: PIN,
	) -> LEDController<CHANNEL, PIN, BUFFER_SIZE, NUM_LEDS> {
		let mut led =
			SmartLedsAdapter::<CHANNEL, PIN, BUFFER_SIZE>::new(channel, pin);
		LEDController {
			led,
			data: [RGB8 {
				r: 255,
				g: 255,
				b: 255,
			}; NUM_LEDS],
		}
	}

	pub fn set_hue_all(&mut self, hue: u8) {
		let color = Hsv {
			hue,
			sat: 255,
			val: 255,
		};
		for i in 0..NUM_LEDS {
			self.data[i] = hsv2rgb(color);
		}
	}

	pub fn show(&mut self) {
		self.led
			.write(brightness(gamma(self.data.iter().cloned()), 10))
			.unwrap();
	}
}
