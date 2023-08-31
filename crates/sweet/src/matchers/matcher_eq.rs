use super::*;
use anyhow::Result;
use std::fmt::Debug;

impl<T> Matcher<T>
where
	T: PartialEq + Debug + Clone,
{
	pub fn to_be(&self, other: T) -> Result<()> { self.assert_equal(other) }

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
}
