use super::*;
use anyhow::Result;

impl Matcher<Result<()>> {
	pub fn to_be_ok(&self) -> Result<()> {
		let result = self.value.is_ok();
		let expected = format!("OK");
		self.assert_correct(result, &expected)
	}
	pub fn to_be_err(&self) -> Result<()> {
		let result = self.value.is_err();
		let expected = format!("Error");
		self.assert_correct(result, &expected)
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
