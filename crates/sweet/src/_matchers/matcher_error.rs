use crate::*;
use anyhow::anyhow;
use anyhow::Result;
use colorize::*;
use std::fmt::Debug;

impl<T> Matcher<T> {
	// useful as seperate from to_be, preserves backtrace depth

	pub fn assert_option_with_received<T2: Debug, T3: Debug>(
		&self,
		expected: &T2,
		received: Option<T3>,
	) -> Result<Option<T3>>
	where
		T3: Debug,
	{
		match received {
			Some(received) => {
				if self.negated {
					Err(self.to_error_with_received(expected, &received))
				} else {
					Ok(Some(received))
				}
			}
			None => {
				if self.negated {
					Ok(None)
				} else {
					Err(self.to_error_with_received(expected, &received))
				}
			}
		}
	}

	pub fn assert_correct_with_received<T2: Debug, T3: Debug>(
		&self,
		result: bool,
		expected: &T2,
		received: &T3,
	) -> Result<()> {
		if self.is_true_with_negated(result) {
			Ok(())
		} else {
			Err(self.to_error_with_received(expected, received))
		}
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
	/// Ensure result is true, and check negated
	pub fn assert_correct<T2: Debug>(
		&self,
		result: bool,
		expected: &T2,
	) -> Result<()> {
		if self.is_true_with_negated(result) {
			Ok(())
		} else {
			Err(self.to_error(expected))
		}
	}

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
