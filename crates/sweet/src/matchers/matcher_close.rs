use super::*;
use anyhow::Result;
use extend::ext;

#[ext]
pub impl<T, U> Matcher<T>
where
	T: SweetBorrow<U>,
	U: CloseTo + std::fmt::Debug + Copy,
{
	fn to_be_close_to(&self, expected: impl SweetBorrow<U>) -> Result<()> {
		self.to_be_close_to_with_epsilon(expected, U::default_epsilon())
	}
	fn to_be_close_to_with_epsilon(
		&self,
		expected: impl SweetBorrow<U>,
		epsilon: U,
	) -> Result<()> {
		let received = self.value.sweet_borrow();
		let expected = expected.sweet_borrow();
		let result = U::is_close(received, expected, epsilon);
		let expected = format!("close to {:?}", expected);
		self.assert_correct_with_received(result, &expected, &received)
	}
}
