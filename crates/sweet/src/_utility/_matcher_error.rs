use crate::Matchable;
use std::fmt;

use super::backtracer;
use colorize::*;


#[derive(Debug, Clone)]
pub struct MatcherError {
	pub message: String,
}

impl MatcherError {
	pub fn new<T: Matchable>(expected: T, received: T, backtrace_depth: usize) -> MatcherError {
		MatcherError {
			message: format!(
				"{}{}{}{}\n\n{}",
				"Expected: ",
				expected.to_string().green(),
				"\nReceived: ",
				received.to_string().red(),
				backtracer::file_context_depth(backtrace_depth + 3),
			),
		}
	}
}

impl fmt::Display for MatcherError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.message) }
}

pub type MatcherResult = Result<(), MatcherError>;
