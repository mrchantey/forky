use anyhow::Result;
use core::time::Duration;
use embedded_svc::http::Method;
use embedded_svc::ipv4::ClientSettings;
use embedded_svc::wifi::{
	ClientConfiguration, ClientConnectionStatus, ClientIpStatus, ClientStatus,
	Configuration, Wifi,
};
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;
use std::sync::Arc;

use super::{fetch, start_server};
pub struct Connection {
	pub wifi: EspWifi,
	pub settings: ClientSettings,
}

impl Connection {
	pub fn new(ssid: &str, password: &str) -> Result<Connection> {
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

		// println!("WIFI - status: {:?}", status);

		if let ClientStatus::Started(ClientConnectionStatus::Connected(
			ClientIpStatus::Done(client_settings),
		)) = status.0
		{
			println!(
				"\n\nWIFI - Connected\nWIFI - IP: {:?}\n\n",
				client_settings.ip
			);
			Ok(Connection {
				wifi,
				settings: client_settings,
			})
		} else {
			Err(anyhow::anyhow!("WIFI - Failed to connect in time."))
		}
	}
	pub fn get(&self, url: impl AsRef<str>) -> anyhow::Result<()> {
		fetch(Method::Get, url)
	}

	pub fn start_server(&self) -> anyhow::Result<()> {
		println!("server running at http://{:?}", self.settings.ip);
		start_server()
	}

	// pub fn post(url: impl AsRef<str>) -> anyhow::Result<()> {

	// }
}
