use anyhow::{Ok, Result};
use core::time::Duration;
use embedded_hal::delay::blocking::DelayUs;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{Gpio7, Output};
use esp_idf_hal::ledc::Channel;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::{
	FixedLengthSignal, PinState, Pulse, Transmit, CHANNEL0,
};
use rgb::RGBA;

pub type RGBA8 = RGBA<u8>;

// const num_leds: i32 = 5;
// TODO sk6812 differnt signal speeds?
// https://cdn-shop.adafruit.com/datasheets/WS2812.pdf

#[macro_export]
macro_rules! smartLedAdapter {
	($num_leds: literal ) => {
		//rgbw 4 * 8 + 1 delimeter
		SmartLedsAdapter::<{ $num_leds * 32 + 1 }>
	};
}

pub struct Led;
impl Led {
	/// these are the endians you're looking for
	pub fn as_u32_be(col: &RGBA8) -> u32 {
		((col.g as u32) << 24)
			+ ((col.r as u32) << 16)
			+ ((col.b as u32) << 8)
			+ ((col.a as u32) << 0)
	}
	/// rarely big endian
	pub fn as_u32_le(col: &RGBA8) -> u32 {
		((col.g as u32) << 0)
			+ ((col.r as u32) << 8)
			+ ((col.b as u32) << 16)
			+ ((col.a as u32) << 24)
	}
}

pub struct LedStrip<const NUM_LEDS: usize, const BUFF_LEN: usize> {
	t0h: Pulse,
	t0l: Pulse,
	t1h: Pulse,
	t1l: Pulse,
	tx: Transmit<Gpio7<Output>, CHANNEL0>,
	pub buffer: [RGBA8; NUM_LEDS],
	signal: FixedLengthSignal<BUFF_LEN>,
}



fn ns(nanos: u64) -> Duration { Duration::from_nanos(nanos) }

impl<const NUM_LEDS: usize, const BUFF_LEN: usize>
	LedStrip<NUM_LEDS, BUFF_LEN>
{
	pub fn new(
		pin: Gpio7<Output>,
		channel: CHANNEL0,
	) -> Result<LedStrip<NUM_LEDS, BUFF_LEN>> {
		let config = TransmitConfig::new().clock_divider(1);
		let mut tx = Transmit::new(pin, channel, &config)?;
		//32 * 6 = 192
		let mut signal = FixedLengthSignal::<BUFF_LEN>::new();

		// let rgbs = [0x0000, 0xffff00, 0x00ffff, 0x00ff00, 0xa000ff];

		let ticks_hz = tx.counter_clock()?;
		let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(350))?;
		let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(800))?;
		let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(700))?;
		let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(600))?;
		Ok(LedStrip {
			t0h,
			t0l,
			t1h,
			t1l,
			tx,
			signal,
			buffer: [RGBA::<u8> {
				r: 0,
				g: 0,
				b: 0,
				a: 0,
			}; NUM_LEDS],
		})
	}
#[rustfmt::skip]
	pub fn show(&mut self) -> Result<()> {
		for (i_col,col) in self.buffer.iter().enumerate() {
			let i_byte = i_col * 32;
			Self::set_channel(&mut self.signal, i_byte, col.g, self.t0l, self.t0h, self.t1l, self.t1h)?;
			Self::set_channel(&mut self.signal, i_byte + 8, col.r, self.t0l, self.t0h, self.t1l, self.t1h)?;
			Self::set_channel(&mut self.signal, i_byte + 16, col.b, self.t0l, self.t0h, self.t1l, self.t1h)?;
			Self::set_channel(&mut self.signal, i_byte + 24, col.a, self.t0l, self.t0h, self.t1l, self.t1h)?;
		}
		//terminator?
		// self.signal.set(self.buffer.len() * 32, &(self.t0h, self.t0l))?;
		self.tx.start_blocking(&self.signal)?;
		Ok(())
	}

	pub fn set_channel(
		signal: &mut FixedLengthSignal<BUFF_LEN>,
		i_byte: usize,
		byte: u8,
		t0l: Pulse,
		t0h: Pulse,
		t1l: Pulse,
		t1h: Pulse,
	) -> Result<()> {
		// let i_channel = i_col * 8;
		for (i_bit, position) in
			[128, 64, 32, 16, 8, 4, 2, 1].iter().enumerate()
		{
			let bit = byte & position;
			let (high_pulse, low_pulse) =
				if bit != 0 { (t1h, t1l) } else { (t0h, t0l) };
			signal.set(i_byte + i_bit, &(high_pulse, low_pulse))?;
		}
		Ok(())
	}

	pub fn set_all(&mut self, r: u8, g: u8, b: u8, w: u8) {
		for mut col in &mut self.buffer {
			col.r = r;
			col.g = g;
			col.b = b;
			col.a = w;
		}
	}
}
