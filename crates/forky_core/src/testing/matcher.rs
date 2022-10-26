use colorize::*;
use std::cmp;
use std::fmt;

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
impl Matchable for &str {}
// impl Matchable for String {}

pub struct Matcher<T: Matchable + cmp::PartialEq + fmt::Display> {
	value: T,
	negated: bool,
}

impl Matcher<bool> {
	pub fn to_be_true(&self) { self.equality_or_panic(true); }
	pub fn to_be_false(&self) { self.equality_or_panic(false); }
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

	pub fn equality_or_panic(&self, other: T) {
		if !self.equality(other) {
			let mut expected = self.value.to_string();
			if self.negated {
				expected.push_str(" (not)");
			}
			panic!(
				"{}{}{}{}",
				"Expected: ".bold(),
				expected.bold().green(),
				"\nReceived: ".bold(),
				other.to_string().bold().red()
			);
		}
	}

	pub fn not(&mut self) -> &mut Self {
		self.negated = !self.negated;
		self
	}

	pub fn to_be(&self, other: T) -> MatcherResult {
		if !self.equality(other) {
			Err(MatcherError {
				message: format!(
					"{}{}{}{}",
					"Expected: ".bold(),
					self.value.to_string().bold().green(),
					"\nReceived: ".bold(),
					other.to_string().bold().red()
				),
			})
		} else {
			Ok(())
		}
	}
}


pub fn expect<T: Matchable + cmp::PartialEq + fmt::Display>(value: T) -> Matcher<T> {
	Matcher::new(value)
}
