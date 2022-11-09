//https://github.com/ferrous-systems/espressif-trainings/blob/main/intro/http-client/examples/http_client.rs
use core::str;
use embedded_svc::{
	http::{
		client::{Client, Request, RequestWrite, Response},
		Method, Status,
	},
	io::Read,
};
use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration};

/// Advised not to call this directly, if you do make sure your connection doesnt go out of scope!
pub fn fetch(method: Method, url: impl AsRef<str>) -> anyhow::Result<()> {
	//no https
	// let mut client = EspHttpClient::new_default()?;
	//TODO https behind feature, its really big
	let mut client = EspHttpClient::new(&EspHttpClientConfiguration {
		use_global_ca_store: true,
		crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
		..Default::default()
	})?;
	let request = client.request(method, url.as_ref())?;

	// 3. Requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
	// (since we don't send anything - but have to do the writer step anyway)
	// https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
	// If this were a POST request, you'd set a write length > 0 and then writer.do_write(&some_buf);
	// request.into_writer(size)
	let writer = request.into_writer(0)?;

	// 4. Submit our write request and check the status code of the response.
	// Successful http status codes are in the 200..=299 range.

	let mut response = writer.submit()?;
	let status = response.status();
	let mut total_size = 0;

	println!("response code: {}\n", status);

	if let 200..=299 = status {
		// 5. if the status is OK, read response data chunk by chunk into a buffer and print it until done
		let mut buf = [0_u8; 256];
		let mut reader = response.reader();
		loop {
			let Ok(size) = Read::read(&mut reader, &mut buf) else { continue };
			if size == 0 {
				break;
			}
			total_size += size;
			// 6. try converting the bytes into a Rust (UTF-8) string and print it
			let response_text = str::from_utf8(&buf[..size])?;
			println!("{}", response_text);
		}
	} else {
		anyhow::bail!("unexpected response code: {}", status)
	}


	Ok(())
}
