use super::*;
use anyhow::Result;
use std::fmt::Debug;

impl<T> Matcher<T>
where
	T: PartialEq + Debug + Clone,
{
	pub fn to_be(&self, other: T) -> Result<()> { self.assert_equal(other) }

	pub fn not(&mut self) -> &mut Self {
		self.negated = !self.negated;
		self
	}

	pub fn assert_equal(&self, expected: T) -> Result<()> {
		if self.equality(&expected) {
			Ok(())
		} else {
			Err(self.to_error(&expected))
		}
	}

	fn equality(&self, other: &T) -> bool {
		if self.negated {
			self.value != *other
		} else {
			self.value == *other
		}
	}

	// useful as seperate from to_be, preserves backtrace depth
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

	fn is_true_with_negated(&self, received: bool) -> bool {
		if self.negated {
			!received
		} else {
			received
		}
	}
}
