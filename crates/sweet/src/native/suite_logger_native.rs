use crate::*;
use crossterm::*;
use std::io::stdout;
// use std::io::Write;




pub struct SuiteLoggerNative {
	stdout: std::io::Result<gag::BufferRedirect>,
	stderr: std::io::Result<gag::BufferRedirect>,
}


impl SuiteLogger for SuiteLoggerNative {
	fn on_start(start_str: String) -> Self {
		// self.log(self.
		println!("{start_str}");
		let this = SuiteLoggerNative {
			stdout: gag::BufferRedirect::stdout(),
			stderr: gag::BufferRedirect::stderr(),
		};
		this
	}

	fn on_end(self, end_str: String) {
		use std::io::Read;
		let mut log = String::new();
		if self.stdout.is_ok() {
			let mut bb = self.stdout.unwrap();
			bb.read_to_string(&mut log).unwrap();
			drop(bb);
		}
		if self.stderr.is_ok() {
			let mut bb = self.stderr.unwrap();
			bb.read_to_string(&mut log).unwrap();
			drop(bb);
		}
		let mut stdout = stdout();
		stdout.execute(crossterm::cursor::MoveUp(1)).unwrap();
		stdout.execute(crossterm::cursor::MoveToColumn(0)).unwrap();
		println!("{end_str}{log}");
	}
}
