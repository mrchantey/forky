use std::ops::{Deref, DerefMut};

use super::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;




#[derive(Debug)]
pub struct TarotDeck {
	pub cards: Vec<TarotCard>,
}

impl TarotDeck {
	pub fn new() -> Self { Self::default() }
	pub fn shuffled() -> Self {
		let mut deck = Self::default();
		deck.shuffle();
		deck
	}

	pub fn shuffle(&mut self) -> &mut Self {
		self.cards.shuffle(&mut thread_rng());
		self
	}
}

impl Default for TarotDeck {
	fn default() -> Self {
		let mut cards = Vec::with_capacity(78);

		for major in MajorArcana::iter() {
			cards.push(TarotCard::Major(major));
		}

		for suit in MinorSuit::iter() {
			for number in MinorNumber::iter() {
				cards.push(TarotCard::Minor(MinorArcana { number, suit }));
			}
		}

		Self { cards }
	}
}

impl Deref for TarotDeck {
	type Target = Vec<TarotCard>;

	fn deref(&self) -> &Self::Target { &self.cards }
}

impl DerefMut for TarotDeck {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.cards }
}
