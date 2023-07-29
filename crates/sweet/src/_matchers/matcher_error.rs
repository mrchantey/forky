use crate::logging::file_context_depth;
use crate::Matcher;
use anyhow::anyhow;
use colorize::*;
use std::fmt::Debug;


impl<T> Matcher<T> {
	pub fn to_error_with_received<T2: Debug, T3: Debug>(
		&self,
		expected: &T2,
		received: &T3,
	) -> anyhow::Error {
		self.to_error_with_received_and_backtrace(expected, received, 0)
	}

	pub fn to_error_with_received_and_backtrace<T2: Debug, T3: Debug>(
		&self,
		expected: &T2,
		received: &T3,
		backtrace_depth: usize,
	) -> anyhow::Error {
		let mut expected = format!("{:?}", expected)
			.trim_matches('"')
			.to_string()
			.green();

		if self.negated {
			expected = format!("{} {}", "NOT".bold().green(), expected);
		}
		let received = format!("{:?}", received)
			.trim_matches('"')
			.to_string()
			.red();

		let backtrace = file_context_depth(4 + backtrace_depth);
		anyhow!(format!(
			"Expected: {expected}\nReceived: {received}\n\n{backtrace}",
		))
	}
}


impl<T> Matcher<T>
where
	T: Debug,
{
	pub fn to_error<T2: Debug>(&self, expected: &T2) -> anyhow::Error {
		self.to_error_with_received_and_backtrace(expected, &self.value, 0)
	}
	pub fn to_error_with_backtrace<T2: Debug>(
		&self,
		expected: &T2,
		backtrace_depth: usize,
	) -> anyhow::Error {
		self.to_error_with_received_and_backtrace(
			expected,
			&self.value,
			backtrace_depth,
		)
	}
}
