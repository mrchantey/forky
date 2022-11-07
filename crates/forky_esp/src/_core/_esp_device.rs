#[allow(unused_imports)]
use esp32c3_hal::{
	clock::ClockControl,
	pac,
	prelude::*,
	pulse_control::ClockSource,
	timer::TimerGroup,
	utils::{smartLedAdapter, SmartLedsAdapter},
	PulseControl, Rtc, IO,
};
use esp_hal_common::{
	pac::{Peripherals, UART0},
	system::SystemParts,
};

use crate::{Logger, Timer};


pub struct ESPDevice {
	pub pulse: PulseControl,
	pub io: IO,
	pub timer: Timer,
	pub logger: Logger,
	// pub peripherals: Peripherals,
	// pub system: SystemParts,
}


impl ESPDevice {
	pub fn new() -> ESPDevice {
		let peripherals = pac::Peripherals::take().unwrap();
		let mut system = peripherals.SYSTEM.split();

		let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

		let timer = Timer::new(&clocks);
		let logger = Logger::new(peripherals.UART0);

		let mut rtc = Rtc::new(peripherals.RTC_CNTL);
		let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
		let mut wdt0 = timer_group0.wdt;

		let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

		//put in led controller?
		let pulse = PulseControl::new(
			peripherals.RMT,
			&mut system.peripheral_clock_control,
			ClockSource::APB,
			0,
			0,
			0,
		)
		.unwrap();

		// Disable watchdogs
		rtc.swd.disable();
		rtc.rwdt.disable();
		wdt0.disable();
		ESPDevice {
			pulse,
			timer,
			logger,
			io,
		}
	}
}
