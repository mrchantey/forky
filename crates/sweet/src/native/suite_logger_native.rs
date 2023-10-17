use crate::test_suite::*;
use crossterm::*;
use std::io::stdout;

pub struct SuiteLoggerNative {
	stdout: Option<gag::BufferRedirect>,
	stderr: Option<gag::BufferRedirect>,
}


impl SuiteLogger for SuiteLoggerNative {
	fn on_start(start_str: String) -> Self {
		println!("{start_str}");
		let this = SuiteLoggerNative {
			stdout: gag::BufferRedirect::stdout().ok(),
			stderr: gag::BufferRedirect::stderr().ok(),
		};
		this
	}

	fn on_end(self, end_str: String) {
		use std::io::Read;
		let mut log = String::new();
		if let Some(mut bb) = self.stdout {
			bb.read_to_string(&mut log).unwrap();
			drop(bb);
		}
		if let Some(mut bb) = self.stderr {
			bb.read_to_string(&mut log).unwrap();
			drop(bb);
		}
		let mut stdout = stdout();
		stdout.execute(crossterm::cursor::MoveUp(1)).unwrap();
		stdout.execute(crossterm::cursor::MoveToColumn(0)).unwrap();
		if log.is_empty() {
			println!("{end_str}");
		} else {
			println!("{end_str}\n{log}");
		}
	}
}
