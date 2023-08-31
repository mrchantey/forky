use super::*;
use anyhow::Result;

impl<T> Matcher<T>
where
	T: std::fmt::Debug + std::marker::Copy + Into<f64>,
{
	pub fn to_be_close_to(&self, other: T) -> Result<()> {
		self.to_be_close_to_with_epsilon(other, DEFAULT_EPSILON_F64)
	}
	pub fn to_be_close_to_with_epsilon(
		&self,
		expected: T,
		epsilon: f64,
	) -> Result<()> {
		let value = self.value.into();
		let expected = expected.into();
		let result = is_close_f64(value, expected, epsilon);
		let expected = format!("close to {:?}", expected);
		self.assert_correct(result, &expected)
	}
}
