use crate::TestSuiteResult;
use colorize::*;
#[cfg(not(target_arch = "wasm32"))]
use crossterm::*;
use std::io::stdout;
use std::io::Read;
// use std::io::Read;
use std::io::Write;
use std::time::Duration;



pub struct TestLogger {
	filename: &'static str,
	#[cfg(not(target_arch = "wasm32"))]
	stdout: Result<gag::BufferRedirect>,
	#[cfg(not(target_arch = "wasm32"))]
	stderr: Result<gag::BufferRedirect>,
	pub log: String,
}

impl TestLogger {
	pub fn start(file: &'static str) -> Self {
		Self::log_start(file);
		let logger = TestLogger {
			filename: file,
			log: String::new(),
			#[cfg(not(target_arch = "wasm32"))]
			stdout: gag::BufferRedirect::stdout(),
			#[cfg(not(target_arch = "wasm32"))]
			stderr: gag::BufferRedirect::stderr(),
		};

		logger
	}

	fn log_start(filename: &str) {
		#[cfg(target_arch = "wasm32")]
		return;
		let prefix = " RUNS ".black().bold().yellowb();
		let path = pretty_path(filename);
		println!("{prefix} {path}");
	}


	pub fn end(mut self, result: &TestSuiteResult) {
		#[cfg(not(target_arch = "wasm32"))]
		if self.stdout.is_ok() {
			let mut bb = self.stdout.unwrap();
			bb.read_to_string(&mut self.log).unwrap();
			drop(bb);
		}

		#[cfg(not(target_arch = "wasm32"))]
		if self.stderr.is_ok() {
			let mut bb = self.stderr.unwrap();
			bb.read_to_string(&mut self.log).unwrap();
			drop(bb);
		}
		let mut stdout = stdout();

		#[cfg(not(target_arch = "wasm32"))]
		stdout.execute(crossterm::cursor::MoveUp(1)).unwrap();
		#[cfg(not(target_arch = "wasm32"))]
		stdout.execute(crossterm::cursor::MoveToColumn(0)).unwrap();

		let prefix = if result.failed == 0 {
			" PASS ".black().bold().greenb()
		} else {
			" FAIL ".black().bold().redb()
		};
		let path = pretty_path(self.filename);
		let log = self.log;
		let out = format!("{prefix} {path}\n{log}");
		stdout.write(out.as_bytes()).unwrap();
	}

	pub fn log_time(duration: Duration) {
		let millis = duration.as_millis();
		let prefix = "Time:\t\t".bold();
		if millis < 100 {
			println!("{}{} ms\n\n", prefix, millis);
		} else {
			println!("{}{:.2} s\n\n", prefix, millis as f32 * 0.001);
		}
	}
}

fn pretty_path(val: &str) -> String {
	let mut splt = val.split("\\").collect::<Vec<&str>>();
	let _file = splt.pop();
	let file = _file.unwrap_or_default().to_string().bold();
	let path = &splt.join("\\").faint();
	let middle = "\\".to_string().faint();
	format!("{path}{middle}{file}")
}

// fn get_now() -> Duration {
// 	#[cfg(target_arch = "wasm32")]
// 	return Duration::from_millis(
// 		web_sys::window().unwrap().performance().unwrap().now() as u64,
// 	);
// 	#[cfg(not(target_arch = "wasm32"))]
// 	return std::time::Instant::now().elapsed(); //TODO now().elapsed is incorrect
// }
