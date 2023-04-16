use std::fmt::{Debug, Display};

use super::*;


pub const NUM_TAROT_CARDS: u8 = NUM_MAJOR_ARCANA + NUM_MINOR_ARCANA;

#[derive(Debug, Clone, Copy)]
pub enum TarotCard {
	Minor(MinorArcana),
	Major(MajorArcana),
}
impl Display for TarotCard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TarotCard::Minor(minor) => Display::fmt(&minor, f),
			TarotCard::Major(major) => Display::fmt(&major, f),
		}
	}
}

impl TarotCard {
	pub fn index(&self) -> u8 {
		match self {
			TarotCard::Minor(minor) => minor.index(),
			TarotCard::Major(major) => major.index(),
		}
	}
}
