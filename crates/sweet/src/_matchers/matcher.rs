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

pub trait MatcherTrait<T> {
	fn get_matcher(&self) -> &Matcher<T>;
	fn get_matcher_mut(&mut self) -> &mut Matcher<T>;
	fn get_value(&self) -> &T;
	fn get_value_mut(&mut self) -> &mut T;
	fn get_negated(&self) -> bool;
	fn set_negated(&mut self, value: bool);
}
impl<T> MatcherTrait<T> for Matcher<T> {
	fn get_matcher(&self) -> &Matcher<T> { self }
	fn get_matcher_mut(&mut self) -> &mut Matcher<T> { self }
	fn get_value(&self) -> &T { &self.value }
	fn get_value_mut(&mut self) -> &mut T { &mut self.value }
	fn get_negated(&self) -> bool { self.negated }
	fn set_negated(&mut self, value: bool) { self.negated = value; }
}
