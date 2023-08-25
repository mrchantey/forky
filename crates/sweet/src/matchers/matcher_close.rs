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
		self.to_be_close_to_with_epsilon(other, EPSILON)
	}
	pub fn to_be_close_to_with_epsilon(
		&self,
		expected: T,
		epsilon: f64,
	) -> Result<()> {
		let diff: f64 = self.abs_diff(expected).into();
		let result = diff < epsilon;
		let expected = format!("close to {:?}", expected);
		self.assert_correct(result, &expected)
	}

	fn abs_diff(&self, other: T) -> T {
		if self.value > other {
			self.value - other
		} else {
			other - self.value
		}
	}
}
