use super::*;
use anyhow::Result;

impl Matcher<bool> {
	pub fn to_be_true(&self) -> Result<()> { self.assert_equal(true) }
	pub fn to_be_false(&self) -> Result<()> { self.assert_equal(false) }
}
