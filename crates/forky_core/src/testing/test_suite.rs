use crate::testing::*;
use crate::*;
use colorize::*;
use crossterm::*;
use gag::BufferRedirect;
use std::io::{stdout, Read};

pub struct TestSuite<'a> {
	desc: &'a TestSuiteDesc,
	num_tests: u32,
	num_failed: u32,
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
			num_tests: 0,
			num_failed: 0,
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

	// pub fn exec<F>(&mut self, mut func: F)
	// where
	// 	F: FnMut(),
	// {
	// 	self.before_each = func;
	// }

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
		let location = [
			" ".to_string(),
			self.get_location(),
			" > ".to_string(),
			name.to_string(),
		]
		.concat();

		// std::io::print


		self.num_tests = self.num_tests + 1;

		let runs_msg = [" RUNS ".black().bold().yellowb(), location.clone()].concat();
		let pass_msg = [" PASS ".black().bold().greenb(), location.clone()].concat();
		let fail_msg = [" FAIL ".black().bold().redb(), location.clone()].concat();

		let mut buf = BufferRedirect::stdout().unwrap();
		let res = func();
		let mut output = String::new();
		buf.read_to_string(&mut output).unwrap();
		drop(buf);
		let mut stdout = stdout();


		stdout.execute(cursor::MoveUp(1)).expect("");
		stdout.execute(style::Print(runs_msg)).expect("");
		// stdout.execute(style::Print("\n")).expect("");
		stdout.execute(cursor::MoveToColumn(0)).expect("");
		if res.is_ok() {
			stdout.execute(style::Print(pass_msg)).expect("");
			println!("\n{}", { output });
		} else if let Some(err) = res.err() {
			self.num_failed = self.num_failed + 1;
			stdout.execute(style::Print(fail_msg)).expect("");
			println!("\n{}", { output });
			log!(err);
			println!("\n");
		}
	}

	pub fn results(&self) -> TestSuiteResult {
		TestSuiteResult {
			tests: self.num_tests,
			failed: self.num_failed,
			skipped: 0,
		}
	}
}
