use super::backtracer;
use anyhow::anyhow;
use colorize::*;
use std::fmt;


#[derive(Debug, Clone)]
pub struct MatcherError;

impl MatcherError {
	pub fn new<T: fmt::Display>(
		expected: T,
		received: T,
		backtrace_depth: usize,
	) -> anyhow::Error {
		Self::new_with_not(expected, received, false, backtrace_depth)
	}
	pub fn new_with_not<T: fmt::Display>(
		expected: T,
		received: T,
		not: bool,
		backtrace_depth: usize,
	) -> anyhow::Error {
		let expected = if not {
			format!("Not {}", expected)
		} else {
			format!("{}", expected)
		};

		anyhow!(format!(
			"{}{}{}{}\n\n{}",
			"Expected: ",
			expected.green(),
			"\nReceived: ",
			received.to_string().red(),
			backtracer::file_context_depth(backtrace_depth + 3),
		))
	}
}

// impl fmt::Display for MatcherError {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		write!(f, "Message: {}", self.message)
// 	}
// }
