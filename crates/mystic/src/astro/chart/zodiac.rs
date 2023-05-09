use super::*;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

#[derive(
	Default, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter,
)]
pub enum Sign {
	#[default]
	Aries,
	Taurus,
	Gemini,
	Cancer,
	Leo,
	Virgo,
	Libra,
	Scorpio,
	Sagittarius,
	Capricorn,
	Aquarius,
	Pisces,
}

impl Sign {
	pub fn meta(&self) -> SignMeta { self.clone().into() }
	// TODO these could be faster with math
	pub fn element(&self) -> Element { self.meta().element }
	pub fn polarity(&self) -> Polarity { self.meta().polarity }
	pub fn mode(&self) -> Mode { self.meta().mode }
}

impl Into<usize> for &Sign {
	fn into(self) -> usize { *self as usize }
}
impl Into<usize> for Sign {
	fn into(self) -> usize { self as usize }
}

impl TryFrom<usize> for Sign {
	type Error = ();

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		Self::iter().nth(value).ok_or(())
	}
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Polarity {
	#[default]
	Positive,
	Negative,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Element {
	#[default]
	Fire,
	Earth,
	Air,
	Water,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Mode {
	#[default]
	Cardinal,
	Fixed,
	Mutable,
}
