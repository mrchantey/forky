use super::BacktraceFile;
use backtrace::{Backtrace, BacktraceSymbol, Symbol};


const PREFIX_STACK_FRAMES: usize = 5;//was 6
const RUST_INTERNAL_FILE: &str = "library\\core\\src\\ops\\function.rs";

pub fn file_context_depth(frame_depth: usize) -> String {
	let bt = Backtrace::new();
	let min_frame = PREFIX_STACK_FRAMES + frame_depth;
	let frame = &bt.frames()[min_frame..][0];
	let symbol = &frame.symbols()[0];
	if let Some(file) = BacktraceFile::new(symbol) {
		return file.file_context().unwrap_or_default();
	}
	String::new()
}
pub fn file_context() -> String {
	//include this frame
	file_context_depth(1)
}

pub fn trace_all() {
	let bt = Backtrace::new();
	let frames = &bt.frames()[PREFIX_STACK_FRAMES..];
	// bt/.
	for frame in frames {
		for symbol in frame.symbols().iter() {
			if let Some(file) = symbol.filename() {
				if let Some(file) = file.to_str() {
					println!("{}", file);
					// if file.contains(RUST_INTERNAL_FILE) {
					// 	return;
					// }
					// if let Some(line) = symbol.lineno() {
					// 	println!("{}: {}", file, line);
					// }
				}
			}
		}
		// println!("{:?}", symbol);
	}
}
