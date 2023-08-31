use super::*;
use anyhow::Result;

impl Matcher<Result<()>> {
	pub fn to_be_ok(&self) -> Result<()> {
		let result = self.value.is_ok();
		self.assert_correct(result, &"Ok")
	}
	pub fn to_be_err(&self) -> Result<()> {
		let result = self.value.is_err();
		self.assert_correct(result, &"Error")
	}
	pub fn to_be_err_str(&self, value: &str) -> Result<()> {
		let expected = format!("Err({})", value);
		if let Err(err) = &self.value {
			let result = err.to_string() == value;
			self.assert_correct(result, &expected)
		} else {
			self.assert_correct(false, &expected)
		}
	}
}
