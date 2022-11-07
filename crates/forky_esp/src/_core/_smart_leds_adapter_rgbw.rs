// use core::{marker::PhantomData, slice::IterMut};
// use fugit::NanosDuration;

// use esp_hal_common::{
// 	pulse_control::{ConfiguredChannel, OutputChannel, PulseCode, RepeatMode},
// 	utils::smart_leds_adapter::*,
// 	OutputPin,
// };


// use smart_leds::RGBA;
// pub type RGBA8 = RGBA<u8>;

// pub struct SmartLedsAdapterRGBW<CHANNEL, PIN, const BUFFER_SIZE: usize> {
// 	channel: CHANNEL,
// 	rmt_buffer: [u32; BUFFER_SIZE],
// 	_pin: PhantomData<PIN>,
// }

// impl<CHANNEL, PIN, const BUFFER_SIZE: usize>
// 	SmartLedsAdapterRGBW<CHANNEL, PIN, BUFFER_SIZE>
// where
// 	CHANNEL: ConfiguredChannel,
// 	PIN: OutputPin,
// {
// 	pub fn new<UnconfiguredChannel>(
// 		mut channel: UnconfiguredChannel,
// 		pin: PIN,
// 	) -> SmartLedsAdapterRGBW<CHANNEL, PIN, BUFFER_SIZE>
// 	where
// 		UnconfiguredChannel: OutputChannel<CHANNEL>,
// 	{
// 		Self {
// 			channel:
// 				SmartLedsAdapter::<CHANNEL, PIN, BUFFER_SIZE>::configure_channel(
// 					channel, pin,
// 				),
// 			rmt_buffer: [0; BUFFER_SIZE],
// 			_pin: PhantomData,
// 		}
// 	}

// 	fn convert_rgbw_to_pulse(
// 		value: RGBA8,
// 		mut_iter: &mut IterMut<u32>,
// 	) -> Result<(), LedAdapterError> {
// 		SmartLedsAdapter::<CHANNEL, PIN, BUFFER_SIZE>::convert_rgb_channel_to_pulses(
// 				value.g, mut_iter,
// 				)?;
// 		SmartLedsAdapter::<CHANNEL, PIN, BUFFER_SIZE>::convert_rgb_channel_to_pulses(
// 					value.r, mut_iter,
// 				)?;
// 		SmartLedsAdapter::<CHANNEL, PIN, BUFFER_SIZE>::convert_rgb_channel_to_pulses(
// 					value.b, mut_iter,
//         )?;
// 		SmartLedsAdapter::<CHANNEL, PIN, BUFFER_SIZE>::convert_rgb_channel_to_pulses(
// 				value.a, mut_iter,
// 				)?;
// 		Ok(())
// 	}

// 	pub fn write<T>(&mut self, iterator: T) -> Result<(), LedAdapterError>
// 	where
// 		T: Iterator<Item = RGBA8>,
// 	{
// 		let mut seq_iter = self.rmt_buffer.iter_mut();
// 		for item in iterator {
// 			SmartLedsAdapterRGBW::<CHANNEL, PIN, BUFFER_SIZE>::convert_rgbw_to_pulse(
//                 item,
//                 &mut seq_iter,
//             )?;
// 		}
// 		*seq_iter.next().ok_or(LedAdapterError::BufferSizeExceeded)? = 0;

// 		match self
// 			.channel
// 			.send_pulse_sequence_raw(RepeatMode::SingleShot, &self.rmt_buffer)
// 		{
// 			Ok(_) => Ok(()),
// 			Err(x) => Err(LedAdapterError::TransmissionError(x)),
// 		}
// 	}
// }
