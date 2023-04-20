use super::super::*;
use crate::astro::{
	chart::{Element, Sign},
	planets::Planet,
};

impl TarotCardMeta {
	// Ace of Wands - Aries
	pub const ACE_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::ACE_OF_WANDS,
		planet: None,
		sign: Some(Sign::Aries),
		element: Element::Fire,
	};

	// 2 of Wands - Mars in Aries
	pub const TWO_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TWO_OF_WANDS,
		planet: Some(Planet::Mars),
		sign: Some(Sign::Aries),
		element: Element::Fire,
	};

	// 3 of Wands - Sun in Aries
	pub const THREE_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THREE_OF_WANDS,
		planet: Some(Planet::Sun),
		sign: Some(Sign::Aries),
		element: Element::Fire,
	};

	// 4 of Wands - Venus in Aries
	pub const FOUR_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::FOUR_OF_WANDS,
		planet: Some(Planet::Venus),
		sign: Some(Sign::Aries),
		element: Element::Fire,
	};

	// 5 of Wands - Saturn in Leo
	pub const FIVE_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::FIVE_OF_WANDS,
		planet: Some(Planet::Saturn),
		sign: Some(Sign::Leo),
		element: Element::Fire,
	};

	// 6 of Wands - Jupiter in Leo
	pub const SIX_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::SIX_OF_WANDS,
		planet: Some(Planet::Jupiter),
		sign: Some(Sign::Leo),
		element: Element::Fire,
	};

	// 7 of Wands - Mars in Leo
	pub const SEVEN_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::SEVEN_OF_WANDS,
		planet: Some(Planet::Mars),
		sign: Some(Sign::Leo),
		element: Element::Fire,
	};

	// 8 of Wands - Mercury in Sagittarius
	pub const EIGHT_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::EIGHT_OF_WANDS,
		planet: Some(Planet::Mercury),
		sign: Some(Sign::Sagittarius),
		element: Element::Fire,
	};

	// 9 of Wands - Moon in Sagittarius
	pub const NINE_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::NINE_OF_WANDS,
		planet: Some(Planet::Moon),
		sign: Some(Sign::Sagittarius),
		element: Element::Fire,
	};

	// 10 of Wands - Saturn in Sagittarius
	pub const TEN_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TEN_OF_WANDS,
		planet: Some(Planet::Moon),
		sign: Some(Sign::Sagittarius),
		element: Element::Fire,
	};
	pub const PAGE_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::PAGE_OF_WANDS,
		planet: None,
		sign: None,
		element: Element::Fire,
	};
	pub const KNIGHT_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::KNIGHT_OF_WANDS,
		planet: None,
		sign: None,
		element: Element::Fire,
	};
	pub const QUEEN_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::QUEEN_OF_WANDS,
		planet: None,
		sign: None,
		element: Element::Fire,
	};
	pub const KING_OF_WANDS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::KING_OF_WANDS,
		planet: None,
		sign: None,
		element: Element::Fire,
	};
}
