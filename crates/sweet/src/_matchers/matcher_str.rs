use super::*;
use anyhow::Result;

impl Matcher<&str> {
	pub fn to_contain(&self, other: &str) -> Result<()> {
		let result = self.value.contains(other);
		let expected = format!("to contain '{}'", other);
		self.assert_correct(result, &expected)
	}
	pub fn to_start_with(&self, other: &str) -> Result<()> {
		let result = self.value.starts_with(other);
		let expected = format!("to start with '{}'", other);
		self.assert_correct(result, &expected)
	}
	pub fn to_end_with(&self, other: &str) -> Result<()> {
		let result = self.value.ends_with(other);
		let expected = format!("to end with '{}'", other);
		self.assert_correct(result, &expected)
	}
}
