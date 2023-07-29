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
}
