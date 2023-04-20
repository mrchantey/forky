use super::*;
use strum_macros::{Display, EnumIter};

pub const NUM_MINOR_NUMBERS: u8 = 14;
pub const NUM_MINOR_SUITS: u8 = 4;
pub const NUM_MINOR_ARCANA: u8 = NUM_MINOR_NUMBERS * NUM_MINOR_SUITS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinorArcana {
	pub number: MinorNumber,
	pub suit: MinorSuit,
}


impl std::fmt::Display for MinorArcana {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "The {} of {}", self.number, self.suit)
	}
}


impl MinorArcana {
	pub fn index(&self) -> u8 {
		NUM_MAJOR_ARCANA
			+ (self.suit as u8 * NUM_MINOR_NUMBERS)
			+ (self.number as u8)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum MinorNumber {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Page,
	Knight,
	Queen,
	King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum MinorSuit {
	Wands,
	Cups,
	Swords,
	Pentacles,
}
