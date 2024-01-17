use super::*;
use std::fmt::Debug;
use std::fmt::Display;

impl<T: Debug, E: Debug> Matcher<Result<T, E>> {
	pub fn to_be_ok(&self) -> anyhow::Result<()> {
		let result = self.value.is_ok();
		self.assert_correct(result, &"Ok")
	}
	pub fn to_be_err(&self) -> anyhow::Result<()> {
		let result = self.value.is_err();
		self.assert_correct(result, &"Error")
	}
}
impl<T: Debug, E: Debug + Display> Matcher<Result<T, E>> {
	pub fn to_be_err_str(&self, value: &str) -> anyhow::Result<()> {
		let expected = format!("Err({})", value);
		if let Err(err) = &self.value {
			let result = err.to_string() == value;
			self.assert_correct(result, &expected)
		} else {
			self.assert_correct(false, &expected)
		}
	}
}
