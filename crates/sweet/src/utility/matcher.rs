use anyhow::Result;
use colorize::AnsiColor;
use std::cmp;
use std::fmt;

use super::MatcherError;
// use super::Result;
// pub type MatchableType = dyn cmp::PartialEq + fmt::Display + std::marker::Copy;
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
	pub fn to_be_true(&self) -> Result<()> { self.assert_equal(true) }
	pub fn to_be_false(&self) -> Result<()> { self.assert_equal(false) }
}
impl Matcher<&str> {
	pub fn to_contain(&self, other: &str) -> Result<()> {
		self.assert_contains(other)
	}
	fn assert_contains(&self, other: &str) -> Result<()> {
		if self.value.contains(other) {
			Ok(())
		} else {
			let expect =
				[String::from("(contains)\n").blue().as_str(), other].join("");
			let receive = ["\n", self.value].join("");
			Err(MatcherError::new(expect.as_str(), receive.as_str(), 0))
		}
	}
}
const EPSILON: f32 = 0.1;

impl Matcher<f32> {
	pub fn to_be_close_to(&self, other: f32) -> Result<()> {
		self.assert_close(other)
	}
	fn assert_close(&self, other: f32) -> Result<()> {
		if (self.value - other).abs() < EPSILON {
			Ok(())
		} else {
			let expect = format!("close to {}", other);
			let receive = format!("{}", self.value);
			Err(MatcherError::new(expect.as_str(), receive.as_str(), 0))
		}
	}
}
impl Matcher<i32> {
	pub fn to_be_at_least(&self, other: i32) -> Result<()> {
		self.assert_greater_equal(other)
	}
	fn assert_greater_equal(&self, other: i32) -> Result<()> {
		if self.value >= other {
			Ok(())
		} else {
			let expect = format!("at least {}", other);
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

	fn assert_equal(&self, other: T) -> Result<()> {
		if self.equality(other) {
			Ok(())
		} else {
			Err(MatcherError::new(other, self.value, 0))
		}
	}

	pub fn to_be(&self, other: T) -> Result<()> { self.assert_equal(other) }
}


pub fn expect<T: Matchable + cmp::PartialEq + fmt::Display>(
	value: T,
) -> Matcher<T> {
	Matcher::new(value)
}

