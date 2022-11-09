use core::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::FromValueType;
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_sys as _;
use std::sync::Arc;

use embedded_svc::wifi::{
	ClientConfiguration, ClientConnectionStatus, ClientIpStatus, ClientStatus,
	Configuration, Wifi,
};

use anyhow::Result;

fn test_wifi(ssid: &str, password: &str) -> Result<String> {
	let netif_stack = Arc::new(EspNetifStack::new()?);
	let sys_look_stack = Arc::new(EspSysLoopStack::new()?);
	let nvs = Arc::new(EspDefaultNvs::new()?);

	let mut wifi = EspWifi::new(netif_stack, sys_look_stack, nvs)?;

	wifi.set_configuration(&Configuration::Client(ClientConfiguration {
		ssid: ssid.into(),
		password: password.into(),
		..Default::default()
	}))?;

	println!("WIFI - connecting to {}...", ssid);
	wifi.wait_status_with_timeout(Duration::from_secs(30), |s| {
		!s.is_transitional()
	})
	.map_err(|e| anyhow::anyhow!("Wait timeout: {:?}", e))?;

	let status = wifi.get_status();

	println!("WIFI - status: {:?}", status);

	if let ClientStatus::Started(ClientConnectionStatus::Connected(
		ClientIpStatus::Done(client_settings),
	)) = status.0
	{
		println!("WIFI - IP: {:?}", client_settings.ip);
		Ok(format!("{:?}", client_settings.ip))
	} else {
		Err(anyhow::anyhow!("WIFI - Failed to connect in time."))
	}
}

fn main() {
	let wifi = test_wifi("royalewithcheese", "tastyburger");
	let ip = match wifi {
		Err(e) => {
			println!("Wifi error: {:?}", e);
			format!("ERR: {:?}", e)
		}
		Ok(s) => s,
	};
}
