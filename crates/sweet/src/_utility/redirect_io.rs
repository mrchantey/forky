use gag::BufferRedirect;
use std::io::Read;


pub fn start_redirect_io() -> BufferRedirect {
	let buf = BufferRedirect::stdout().unwrap();
	buf
}

pub fn end_redirect_io(buf: &mut BufferRedirect) -> String {
	let mut output = String::new();
	buf.read_to_string(&mut output).unwrap();
	output
}
