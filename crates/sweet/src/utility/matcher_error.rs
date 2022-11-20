use crate::Matchable;
use std::fmt::{self};

use super::backtracer;
use anyhow::anyhow;
use colorize::*;


#[derive(Debug, Clone)]
pub struct MatcherError;

impl MatcherError {
	pub fn new<T: Matchable>(
		expected: T,
		received: T,
		backtrace_depth: usize,
	) -> anyhow::Error {
		anyhow!(format!(
			"{}{}{}{}\n\n{}",
			"Expected: ",
			expected.to_string().green(),
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
