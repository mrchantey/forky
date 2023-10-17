use super::*;
use anyhow::Result;
use extend::ext;

#[ext(name=MatcherExtClose)]
/// Matcher Extensions for types that implement `CloseTo`: `f32`, `f64`, `Vec3`, etc.
pub impl<T, U> Matcher<T>
where
	T: SweetBorrow<U>,
	U: CloseTo + std::fmt::Debug + Copy,
{
	fn to_be_close_to(&self, expected: impl SweetBorrow<U>) -> Result<()> {
		let received = self.value.sweet_borrow();
		let expected = expected.sweet_borrow();
		let result = U::is_close(received, expected, U::default_epsilon());
		let expected = format!("close to {:?}", expected);
		self.assert_correct_with_received(result, &expected, &received)
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
