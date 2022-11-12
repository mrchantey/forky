use crate::EspHttpRequest_Ext;
use anyhow::Result;
use esp_idf_svc::http::server::EspHttpRequest;

pub const MAX_SKETCH_SIZE: usize = 1024;

pub struct SketchBuffer {
	pub dirty: bool,
	pub buffer: [u8; MAX_SKETCH_SIZE],
	pub len: usize,
}

impl SketchBuffer {
	pub fn new() -> SketchBuffer {
		SketchBuffer {
			dirty: false,
			buffer: [0; MAX_SKETCH_SIZE],
			len: 0,
		}
	}

	pub fn from_request(&mut self, request: &mut EspHttpRequest) -> Result<()> {
		self.len = request.read_bytes_to_arr(&mut self.buffer)?;
		self.dirty = true;
		Ok(())
	}
}
