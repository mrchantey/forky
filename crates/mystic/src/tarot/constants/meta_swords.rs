use super::super::*;
use crate::astro::{
	chart::{Element, Sign},
	planets::Planet,
};

impl TarotCardMeta {
	// Ace of Swords - Aquarius
	pub const ACE_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::ACE_OF_SWORDS,
		planet: Some(Planet::Uranus),
		sign: Some(Sign::Aquarius),
		element: Element::Air,
	};

	// 2 of Swords - Moon in Libra
	pub const TWO_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TWO_OF_SWORDS,
		planet: Some(Planet::Moon),
		sign: Some(Sign::Libra),
		element: Element::Air,
	};

	// 3 of Swords - Saturn in Libra
	pub const THREE_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THREE_OF_SWORDS,
		planet: Some(Planet::Saturn),
		sign: Some(Sign::Libra),
		element: Element::Air,
	};

	// 4 of Swords - Jupiter in Libra
	pub const FOUR_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::FOUR_OF_SWORDS,
		planet: Some(Planet::Jupiter),
		sign: Some(Sign::Libra),
		element: Element::Air,
	};

	// 5 of Swords - Venus in Aquarius
	pub const FIVE_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::FIVE_OF_SWORDS,
		planet: Some(Planet::Venus),
		sign: Some(Sign::Aquarius),
		element: Element::Air,
	};

	// 6 of Swords - Mercury in Aquarius
	pub const SIX_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::SIX_OF_SWORDS,
		planet: Some(Planet::Mercury),
		sign: Some(Sign::Aquarius),
		element: Element::Air,
	};

	// 7 of Swords - Moon in Aquarius
	pub const SEVEN_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::SEVEN_OF_SWORDS,
		planet: Some(Planet::Moon),
		sign: Some(Sign::Aquarius),
		element: Element::Air,
	};

	// 8 of Swords - Jupiter in Gemini
	pub const EIGHT_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::EIGHT_OF_SWORDS,
		planet: Some(Planet::Jupiter),
		sign: Some(Sign::Gemini),
		element: Element::Air,
	};

	// 9 of Swords - Mars in Gemini
	pub const NINE_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::NINE_OF_SWORDS,
		planet: Some(Planet::Mars),
		sign: Some(Sign::Gemini),
		element: Element::Air,
	};

	// 10 of Swords - Sun in Gemini
	pub const TEN_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TEN_OF_SWORDS,
		planet: Some(Planet::Sun),
		sign: Some(Sign::Gemini),
		element: Element::Air,
	};
	pub const PAGE_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::PAGE_OF_SWORDS,
		planet: None,
		sign: None,
		element: Element::Air,
	};
	pub const KNIGHT_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::KNIGHT_OF_SWORDS,
		planet: None,
		sign: None,
		element: Element::Air,
	};
	pub const QUEEN_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::QUEEN_OF_SWORDS,
		planet: None,
		sign: None,
		element: Element::Air,
	};
	pub const KING_OF_SWORDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::KING_OF_SWORDS,
		planet: None,
		sign: None,
		element: Element::Air,
	};
}
