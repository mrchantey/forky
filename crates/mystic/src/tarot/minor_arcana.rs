use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumIter};

pub const NUM_MINOR_ARCANA: usize = MinorNumber::COUNT * MinorSuit::COUNT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinorArcana {
	pub number: MinorNumber,
	pub suit: MinorSuit,
}


impl std::fmt::Display for MinorArcana {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "The {} of {}", self.number, self.suit)
	}
}

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount, Display, Hash,
)]
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

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount, Display, Hash,
)]
pub enum MinorSuit {
	Wands,
	Cups,
	Swords,
	Pentacles,
}
