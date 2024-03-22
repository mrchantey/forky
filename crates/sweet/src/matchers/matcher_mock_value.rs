use super::*;
use crate::*;
use anyhow::Result;
use std::fmt::Debug;

impl<T> Matcher<&MockValue<T>> {
	pub fn to_be_empty(&self) -> Result<()> {
		let received = self.value.pop();
		self.assert_correct_with_received(
			received.is_none(),
			&"to have been called",
			&false,
		)
	}
}
impl<T: Debug + PartialEq> Matcher<&MockValue<T>> {
	/// checks the first time it was called
	pub fn to_contain(&self, expected: T) -> Result<()> {
		if let Some(received) = &self.value.pop() {
			self.assert_correct_with_received(
				received == &expected,
				&expected,
				received,
			)
		} else {
			Err(self.to_error_with_received(&"to have been called", &false))
		}
	}
}

//TODO to_have_been_called_with
