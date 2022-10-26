use crate::testing::*;
use crate::utility;
use crate::*;
use colorize::*;
use crossterm::*;
use gag::BufferRedirect;
use std::io::{stdout, Read, Write};

pub struct TestSuite<'a> {
	desc: &'a TestSuiteDesc,
	num_tests: u32,
	num_failed: u32,
	num_skipped: u32,
	log: String,
	// logger: TestLogger<'a>,
}

#[derive(Default)]
pub struct TestSuiteResult {
	pub tests: u32,
	pub failed: u32,
	pub skipped: u32,
}

// const myFunc: 'static dyn FnMut() = ||{};

impl<'a> TestSuite<'a> {
	pub fn new(desc: &'a TestSuiteDesc) -> Self {
		TestSuite {
			desc,
			log: String::new(),
			// logger: TestLogger::new(desc.name, desc.file),
			num_tests: 0,
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
		["", path, &middle, &file, " > ", self.desc.name].concat()
	}

	pub fn skip(&mut self) -> &mut Self {
		self.num_skipped = self.num_skipped + 1;
		self
	}

	pub fn it<F>(&mut self, name: &'a str, func: F)
	where
		F: FnOnce() -> MatcherResult,
	{
		self.test(name, func);
	}
	pub fn test<F>(&mut self, name: &'a str, func: F)
	where
		F: FnOnce() -> MatcherResult,
	{
		let location = [" ", &self.get_location()[..], " > ", name].concat();

		self.num_tests = self.num_tests + 1;


		let mut buf = BufferRedirect::stdout().unwrap();
		let res = func();
		// let mut output = String::new();
		buf.read_to_string(&mut self.log).unwrap();
		drop(buf);
		if let Some(err) = res.err() {
			self.num_failed = self.num_failed + 1;
			self.log.push_str(&err.message[..]);
		}
	}

	pub fn print_runs(&self) {
		let location = self.get_location();
		let runs_msg = [&" RUNS ".black().bold().yellowb()[..], " ", &location[..]].concat();
		log!(runs_msg);
	}

	pub fn print_log(&self) {
		let mut stdout = stdout();

		stdout.execute(cursor::MoveUp(1)).unwrap();
		stdout.execute(cursor::MoveToColumn(0)).unwrap();

		let mut prefix = if self.num_failed == 0 {
			" PASS ".black().bold().greenb()
		} else {
			" FAIL ".black().bold().redb()
		};
		prefix.push_str(&self.get_location()[..]);
		stdout.write(prefix.as_bytes()).unwrap();
		stdout.write(self.log.as_bytes()).unwrap();
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
