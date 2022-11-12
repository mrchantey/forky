use anyhow::Result;
use embedded_svc::{
	http::server::{Request, Response},
	io::{Read, Write},
};
use esp_idf_svc::http::server::{EspHttpRequest, EspHttpResponse};
use extend::ext;
#[ext]
pub impl EspHttpRequest<'_> {
	fn read_bytes_to_arr<const T: usize>(
		&mut self,
		buff: &mut [u8; T],
	) -> Result<usize> {
		let len = self.reader().read(buff)?;
		Ok(len)
	}
	fn read_bytes<const T: usize>(&mut self) -> Result<([u8; T], usize)> {
		let mut buff = [0; T];
		let len = self.read_bytes_to_arr(&mut buff)?;
		Ok((buff, len))
	}
}

#[ext]
pub impl EspHttpResponse<'_> {
	fn write_bytes(mut self, bytes: &[u8]) -> Result<()> {
		let mut writer = self.into_writer()?;
		writer.write_all(bytes)?;
		Ok(())
	}

	fn ok(mut self) -> Result<()> { self.write_bytes(b"ok") }
}
