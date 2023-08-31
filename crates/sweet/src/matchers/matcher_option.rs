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
	pub fn to_be_none(&self) -> Result<()> {
		let result = self.value.is_none();
		self.assert_correct(result, &"None")
	}
}
