use colorize::*;
use std::cmp;
use std::fmt;

use super::Backtracer;
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
	pub fn to_be_true(&self) -> MatcherResult { self.to_be(true) }
	pub fn to_be_false(&self) -> MatcherResult { self.to_be(false) }
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

	pub fn to_be(&self, other: T) -> MatcherResult {
		if !self.equality(other) {
			// Backtracer::trace_all();
			Err(MatcherError {
				message: format!(
					"{}{}{}{}\n\n{}",
					"Expected: ",
					self.value.to_string().green(),
					"\nReceived: ",
					other.to_string().red(),
					Backtracer::file_context_depth(1),
					// Backtracer::file_context_depth(2),
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
