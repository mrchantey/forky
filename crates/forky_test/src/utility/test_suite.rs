use forky_core::*;
use crate::*;
use colorize::*;
use crossterm::*;
use gag::BufferRedirect;
use std::io::{stdout, Read, Write};

pub struct TestSuite {
	desc: &'static TestSuiteDesc,
	num_tests: u32,
	num_failed: u32,
	num_skipped: u32,
	log: String,
	skip_next_test: bool,
	pub quiet: bool,
	// logger: TestLogger<'a>,
}

#[derive(Default)]
pub struct TestSuiteResult {
	pub tests: u32,
	pub failed: u32,
	pub skipped: u32,
}

// const myFunc: 'static dyn FnMut() = ||{};

impl TestSuite {
	pub fn new(desc: &'static TestSuiteDesc) -> Self {
		TestSuite {
			desc,
			log: String::new(),
			// logger: TestLogger::new(desc.name, desc.file),
			num_tests: 0,
			quiet: false,
			skip_next_test: false,
			num_failed: 0,
			num_skipped: 0,
		}
	}

	fn get_location(&self) -> String {
		let mut splt = self.desc.file.split("\\").collect::<Vec<&str>>();
		let _file = splt.pop();
		let file = _file.or_default().to_string().bold();
		let path = &splt.join("\\").faint();
		let middle = "\\".to_string().faint();
		["", path, &middle, &file].concat()
	}

	pub fn skip(&mut self) -> &mut Self {
		self.skip_next_test = true;
		self
	}

	pub fn it<F>(&mut self, name: &str, func: F)
	where
		F: FnOnce() -> MatcherResult,
	{
		self.test(name, func);
	}
	pub fn test<F>(&mut self, name: &str, func: F)
	where
		F: FnOnce() -> MatcherResult,
	{
		self.num_tests = self.num_tests + 1;
		if self.skip_next_test {
			self.num_skipped = self.num_skipped + 1;
			self.skip_next_test = false;
			return;
		}

		let buf = BufferRedirect::stdout();
		let res = func();
		if buf.is_ok(){
			let mut bb = buf.unwrap();
			bb.read_to_string(&mut self.log).unwrap();
			drop(bb);
		}

		if let Some(err) = res.err() {
			self.num_failed = self.num_failed + 1;
			self.log.push_str(
				&["\n● ", self.desc.name, " > ", name, "\n\n"]
					.concat()
					.red()
					.bold()[..],
			);
			self.log.push_str(&err.message[..]);
			self.log.push_str("\n\n");
		}
	}

	pub fn print_runs(&self) {
		let location = self.get_location();
		let runs_msg = [&" RUNS ".black().bold().yellowb()[..], " ", &location[..]].concat();
		log!(runs_msg);
	}

	pub fn print_log(&self) {
		if self.quiet {
			return;
		}
		let mut stdout = stdout();

		stdout.execute(cursor::MoveUp(1)).unwrap();
		stdout.execute(cursor::MoveToColumn(0)).unwrap();

		let mut prefix = if self.num_failed == 0 {
			" PASS ".black().bold().greenb()
		} else {
			" FAIL ".black().bold().redb()
		};
		prefix.push_str(&[&" ", &self.get_location()[..], "\n", &self.log[..]].concat()[..]);
		stdout.write(prefix.as_bytes()).unwrap();
	}

	pub fn results(&self) -> TestSuiteResult {
		self.print_log();
		TestSuiteResult {
			tests: self.num_tests,
			failed: self.num_failed,
			skipped: self.num_skipped,
		}
	}
}
