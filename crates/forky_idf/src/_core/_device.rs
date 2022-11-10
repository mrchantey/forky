use esp_idf_hal::{prelude::*, rmt::config::TransmitConfig};

pub struct IDFDevice {
	pub peripherals: Peripherals,
}

impl IDFDevice {
	pub fn new() -> IDFDevice {
		let peripherals = Peripherals::take().unwrap();
		let config = TransmitConfig::new().clock_divider(1);
		IDFDevice { peripherals }
	}
}
