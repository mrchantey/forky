use super::*;
use anyhow::Result;

impl<T> Matcher<T>
where
	T: PartialOrd + std::fmt::Debug + std::marker::Copy,
{
	pub fn to_be_less_than(&self, other: T) -> Result<()> {
		if self.value < other {
			Ok(())
		} else {
			let expect = format!("less than {:?}", other);
			let receive = format!("{:?}", self.value);
			Err(MatcherError::new(expect, receive, 0))
		}
	}
	pub fn to_be_less_or_equal_to(&self, other: T) -> Result<()> {
		if self.value <= other {
			Ok(())
		} else {
			let expect = format!("less or equal to {:?}", other);
			let receive = format!("{:?}", self.value);
			Err(MatcherError::new(expect, receive, 0))
		}
	}
	pub fn to_be_greater_than(&self, other: T) -> Result<()> {
		if self.value > other {
			Ok(())
		} else {
			let expect = format!("greater than {:?}", other);
			let receive = format!("{:?}", self.value);
			Err(MatcherError::new(expect, receive, 0))
		}
	}
	pub fn to_be_greater_or_equal_to(&self, other: T) -> Result<()> {
		if self.value >= other {
			Ok(())
		} else {
			let expect = format!("greater or equal to {:?}", other);
			let receive = format!("{:?}", self.value);
			Err(MatcherError::new(expect, receive, 0))
		}
	}
}
