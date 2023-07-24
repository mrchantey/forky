use crate::*;
use crossterm::*;
use std::io::stdout;
use std::io::Write;

pub struct SuiteLoggerNative {
	pub log: String,
	stdout: std::io::Result<gag::BufferRedirect>,
	stderr: std::io::Result<gag::BufferRedirect>,
}


impl SuiteLogger for SuiteLoggerNative {
	fn log(val: &str) {
		let mut stdout = stdout();
		stdout.write(val.as_bytes()).unwrap();
	}
	fn get_log(&mut self) -> &mut String{ &mut self.log }

	fn on_start() -> Self {
		SuiteLoggerNative {
			log: String::from("\n"),
			stdout: gag::BufferRedirect::stdout(),
			stderr: gag::BufferRedirect::stderr(),
		}
	}

	fn on_end(mut self, running_indicator: bool) {
		use std::io::Read;
		if self.stdout.is_ok() {
			let mut bb = self.stdout.unwrap();
			bb.read_to_string(&mut self.log).unwrap();
			drop(bb);
		}
		if self.stderr.is_ok() {
			let mut bb = self.stderr.unwrap();
			bb.read_to_string(&mut self.log).unwrap();
			drop(bb);
		}
		let mut stdout = stdout();
		if running_indicator {
			stdout.execute(crossterm::cursor::MoveUp(1)).unwrap();
			stdout.execute(crossterm::cursor::MoveToColumn(0)).unwrap();
		}

		stdout.write(self.log.as_bytes()).unwrap();
	}
}
