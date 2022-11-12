use std::sync::{Arc, Mutex};

use anyhow::Result;
use embedded_svc::{
	http::server::{registry::Registry, Request, Response},
	io::Read,
	io::Write,
};
use esp_idf_svc::http::server::{Configuration, EspHttpRequest, EspHttpServer};
use forky_wasm::{SketchInstance, WasmEngine};

use crate::*;

pub struct SketchServer {}


impl SketchServer {
	pub fn run() -> Result<()> {
		let wifi = wifi::Connection::new(secret::SSID, secret::PASSWORD)?;
		let mut server = wifi.start_server()?;

		let mut engine = WasmEngine::new();
		let (leds) = default_peripherals()?;
		let mut sketch =
			SketchInstance::from_default_with_engine(&mut engine, &leds);

		let buffer = Arc::new(Mutex::new(SketchBuffer::new()));
		// let engine = Arc::clone(&sketch);
		let buffer2 = Arc::clone(&buffer);
		server.handle_post("/sketch", move |mut request, response| {
			let mut buffer = buffer2.lock().unwrap();
			buffer.from_request(&mut request)?;
			println!("\nsketch received! {}b", buffer.len);

			response.ok()?;
			Ok(())
		})?;
		loop {
			let mut buffer = buffer.lock().unwrap();
			if buffer.dirty {
				// sketch.instance.instance.
				// let mut engine = WasmEngine::new();
				sketch = SketchInstance::new_with_engine(
					&mut engine,
					&buffer.buffer[..buffer.len],
					&leds,
				);
				buffer.dirty = false;
			}
			sketch.run();
			utility::sleep_ms(16);
		}
	}
}
