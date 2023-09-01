use backtrace::BacktraceSymbol;
use colorize::*;
use forky_core::*;
use std::fs;
use std::io;
use std::path::Path;

const LINE_CONTEXT_SIZE: usize = 2;

pub struct BacktraceFile<'a> {
	file: &'a Path,
	file_rel: &'a Path,
	line: u32,
	col: u32,
}

impl<'a> BacktraceFile<'a> {
	pub fn new(symbol: &BacktraceSymbol) -> Option<BacktraceFile> {
		if let Some(file) = symbol.filename() {
			let file_rel = file.relative().unwrap_or(file);
			if let Some(line) = symbol.lineno() {
				if let Some(col) = symbol.colno() {
					return Some(BacktraceFile {
						file,
						file_rel,
						line,
						col,
					});
				} else {
					return Some(BacktraceFile {
						file,
						file_rel,
						line,
						col: 0,
					});
				}
			}
			// }
		}
		return None;
	}
	pub fn file_context(&self) -> io::Result<String> {
		//line number is one-indexed
		let line_no = self.line as usize;
		let col_no = self.col as usize;
		let lines = fs::read_to_string(self.file)?;
		let lines: Vec<&str> = lines.split("\n").collect();
		let start = usize::max(0, line_no - LINE_CONTEXT_SIZE - 1);
		let end = usize::min(lines.len() - 1, line_no + LINE_CONTEXT_SIZE);

		let mut output = String::new();

		for i in start..end {
			let curr_line_no = i + 1;
			let is_err_line = curr_line_no == line_no;
			let prefix = String::from(tern!(is_err_line; ">"; " ")).red();

			let buffer = line_number_buffer(curr_line_no);
			let line_prefix =
				String::from(format!("{}{}|", curr_line_no, buffer)).faint();
			let full_prefix = format!("{} {}", prefix, line_prefix);
			// let prefix_len = 6;
			output.push_string(&full_prefix);
			output.push_str(lines[i]);
			output.push('\n');
			if is_err_line {
				//TODO string length
				output.push_string(
					&format!("{}|", " ".repeat(2 + LINE_BUFFER_LEN)).faint(),
				);
				output.push_str(&" ".repeat(col_no));
				output.push_str_line(String::from("^").red().as_str());
			}
		}
		let prefix = String::from("at").faint();
		let file_loc =
			String::from(self.file_rel.to_str().unwrap_or("unkown file"))
				.cyan();
		let line_loc =
			String::from(format!(":{}:{}", self.line, self.col)).faint();

		output.push_string(&format!("\n{} {}{}\n", prefix, file_loc, line_loc));
		Ok(output)
	}
}

const LINE_BUFFER_LEN: usize = 3;

fn line_number_buffer(line_no: usize) -> String {
	let line_no = line_no.to_string();
	let digits = line_no.len();
	let len = LINE_BUFFER_LEN - digits;
	" ".repeat(len)
}
