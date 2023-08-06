pub fn expect<T>(value: T) -> Matcher<T> { Matcher::new(value) }

pub struct Matcher<T> {
	pub value: T,
	pub negated: bool,
}

impl<T> Matcher<T> {
	pub fn new(value: T) -> Matcher<T> {
		Matcher {
			value,
			negated: false,
		}
	}

	pub fn not(&mut self) -> &mut Self {
		self.negated = !self.negated;
		self
	}

	pub fn is_true_with_negated(&self, received: bool) -> bool {
		if self.negated {
			!received
		} else {
			received
		}
	}
}
