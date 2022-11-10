use anyhow::Result;
use forky_idf::*;

fn main() -> Result<()> {
	let wifi = wifi::Connection::new(secret::SSID, secret::PASSWORD)?;

	wifi.get("http://example.com")?;
	println!("HTTP OK!");
	wifi.get("https://espressif.com")?;
	println!("HTTPS OK!");
	Ok(())
}
