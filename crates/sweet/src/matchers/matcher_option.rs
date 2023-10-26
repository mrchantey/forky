use super::*;
use anyhow::Result;

impl<T> Matcher<Option<T>>
where
	T: std::fmt::Debug,
{
	pub fn to_be_some(&self) -> Result<()> {
		let result = self.value.is_some();
		self.assert_correct(result, &"Some")
	}
	pub fn as_some(self) -> Result<Matcher<T>> {
		if let Some(value) = self.value {
			Ok(Matcher::new(value))
		} else {
			Err(self.to_error_with_backtrace(&"Some", 1))
		}
	}
	pub fn to_be_none(&self) -> Result<()> {
		let result = self.value.is_none();
		self.assert_correct(result, &"None")
	}
}
