use crate::matchers::*;
use crate::test_case::*;
use anyhow::anyhow;
use anyhow::Result;
use colorize::*;
use std::fmt::Debug;

impl<T> Matcher<T> {
	pub fn assert_option_with_received<T2>(
		&self,
		received: Option<T2>,
	) -> Result<T2> {
		self.disallow_negated()?;
		match received {
			Some(r) => Ok(r),
			None => Err(self.to_error_with_received(&"Some", &"None")),
		}
	}

	pub fn assert_option_with_received_negatable<T2>(
		&self,
		received: Option<T2>,
	) -> Result<()> {
		if self.negated && received.is_some() {
			Err(self.to_error_with_received(&"None", &"Some"))
		} else if !self.negated && received.is_none() {
			Err(self.to_error_with_received(&"Some", &"None"))
		} else {
			Ok(())
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

	/// Testing use only
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
		anyhow!("Expected: {expected}\nReceived: {received}\n\n{backtrace}",)
	}
	pub fn to_custom_error(err: &str) -> anyhow::Error {
		Self::to_custom_error_with_backtrace(err, 1)
	}

	pub fn to_custom_error_with_backtrace(
		err: &str,
		backtrace_depth: usize,
	) -> anyhow::Error {
		let backtrace = file_context_depth(4 + backtrace_depth);
		anyhow!("{err}\n\n{backtrace}")
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
