use super::*;
use anyhow::Result;

impl Matcher<&str> {
	pub fn to_contain(&self, other: &str) -> Result<()> {
		if self.value.contains(other) {
			Ok(())
		} else {
			let expect = format!("contains {}", other);
			let receive = self.value;
			Err(MatcherError::new(expect, receive, 0))
		}
	}
}
