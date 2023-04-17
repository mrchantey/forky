use super::*;
use anyhow::Result;

const EPSILON: f64 = 0.1;

impl<T> Matcher<T>
where
	T: PartialEq
		+ PartialOrd
		+ std::ops::Sub<Output = T>
		+ std::fmt::Debug
		+ std::marker::Copy
		+ Into<f64>,
{
	pub fn to_be_close_to(&self, other: T) -> Result<()> {
		if self.abs_diff(other).into() < EPSILON {
			Ok(())
		} else {
			let expect = format!("close to {:?}", other);
			let receive = format!("{:?}", self.value);
			Err(MatcherError::new(expect, receive, 0))
		}
	}

	fn abs_diff(&self, other: T) -> T {
		if self.value > other {
			self.value - other
		} else {
			other - self.value
		}
	}
}
