use std::cmp;
use std::fmt;

use colorize::AnsiColor;

use super::MatcherError;
use super::MatcherResult;
pub trait Matchable: cmp::PartialEq + fmt::Display + std::marker::Copy {}
impl Matchable for bool {}
impl Matchable for f32 {}
impl Matchable for f64 {}
impl Matchable for u8 {}
impl Matchable for u16 {}
impl Matchable for u32 {}
impl Matchable for u64 {}
impl Matchable for u128 {}
impl Matchable for i8 {}
impl Matchable for i16 {}
impl Matchable for i32 {}
impl Matchable for i64 {}
impl Matchable for i128 {}
impl Matchable for usize {}
impl Matchable for char {}
impl Matchable for &str {}
// impl Matchable for String {}

pub struct Matcher<T: Matchable + cmp::PartialEq + fmt::Display> {
	value: T,
	negated: bool,
}

impl Matcher<bool> {
	pub fn to_be_true(&self) -> MatcherResult { self.assert_equal(true) }
	pub fn to_be_false(&self) -> MatcherResult { self.assert_equal(false) }
}
impl Matcher<&str> {
	pub fn to_contain(&self, other: &str) -> MatcherResult { self.assert_contains(other) }
	fn assert_contains(&self, other: &str) -> MatcherResult {
		if self.value.contains(other) {
			Ok(())
		} else {
			let expect = [String::from("(contains)\n").blue().as_str(), other].join("");
			let receive = ["\n", self.value].join("");
			Err(MatcherError::new(expect.as_str(), receive.as_str(), 0))
		}
	}
}
const MIN_DELTA: f32 = 0.1;

impl Matcher<f32> {
	pub fn to_be_close_to(&self, other: f32) -> MatcherResult { self.assert_close(other) }
	fn assert_close(&self, other: f32) -> MatcherResult {
		if (self.value - other).abs() < MIN_DELTA {
			Ok(())
		} else {
			let expect = format!("close to {}", other);
			let receive = format!("{}", self.value);
			Err(MatcherError::new(expect.as_str(), receive.as_str(), 0))
		}
	}
}

impl<T: Matchable> Matcher<T> {
	pub fn new(value: T) -> Matcher<T> {
		Matcher {
			value,
			negated: false,
		}
	}

	pub fn equality(&self, other: T) -> bool {
		if self.negated {
			self.value != other
		} else {
			self.value == other
		}
	}

	pub fn not(&mut self) -> &mut Self {
		self.negated = !self.negated;
		self
	}

	fn assert_equal(&self, other: T) -> MatcherResult {
		if self.equality(other) {
			Ok(())
		} else {
			Err(MatcherError::new(other, self.value, 0))
		}
	}

	pub fn to_be(&self, other: T) -> MatcherResult { self.assert_equal(other) }
}


pub fn expect<T: Matchable + cmp::PartialEq + fmt::Display>(value: T) -> Matcher<T> {
	Matcher::new(value)
}
