use std::cmp;
use std::fmt;
use std::fmt::Debug;

pub fn expect<T>(value: T) -> Matcher<T>
where
	T: cmp::PartialEq + fmt::Debug + std::marker::Copy,
{
	Matcher::new(value)
}


pub struct Matcher<T: cmp::PartialEq + Debug> {
	pub value: T,
	pub negated: bool,
}
