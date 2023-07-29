use super::*;
use anyhow::Result;


impl<T> Matcher<T>
where
	T: PartialEq + std::fmt::Debug + Clone,
{
	pub fn new(value: T) -> Matcher<T> {
		Matcher {
			value,
			negated: false,
		}
	}

	pub fn equality(&self, other: &T) -> bool {
		if self.negated {
			self.value != *other
		} else {
			self.value == *other
		}
	}

	pub fn not(&mut self) -> &mut Self {
		self.negated = !self.negated;
		self
	}

	pub fn assert_equal(&self, other: T) -> Result<()> {
		if self.equality(&other) {
			Ok(())
		} else {
			Err(MatcherError::new_with_not(
				other,
				self.value.clone(),
				self.negated,
				0,
			))
		}
	}

	pub fn to_be(&self, other: T) -> Result<()> { self.assert_equal(other) }
}
