use crate::logging::file_context_depth;
use crate::Matcher;
use anyhow::anyhow;
use colorize::*;
use std::fmt::Debug;

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
		let expected = if self.negated {
			format!("Not {:?}", expected)
		} else {
			format!("{:?}", expected)
		};

		let received = format!("{:?}", received);
		anyhow!(format!(
			"{}{}{}{}\n\n{}",
			"Expected: ",
			expected.green(),
			"\nReceived: ",
			received.red(),
			file_context_depth(4 + backtrace_depth),
		))
	}
}
