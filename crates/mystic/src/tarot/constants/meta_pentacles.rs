use super::super::*;
use crate::astro::{
	chart::{Element, Sign},
	planets::Planet,
};

impl TarotCardMeta {
	// Ace of Pentacles - Capricorn
	pub const ACE_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::ACE_OF_PENTACLES,
		planet: None,
		sign: Some(Sign::Capricorn),
		element: Element::Earth,
	};

	// 2 of Pentacles - Jupiter in Capricorn
	pub const TWO_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TWO_OF_PENTACLES,
		planet: Some(Planet::Jupiter),
		sign: Some(Sign::Capricorn),
		element: Element::Earth,
	};

	// 3 of Pentacles - Mars in Capricorn
	pub const THREE_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THREE_OF_PENTACLES,
		planet: Some(Planet::Mars),
		sign: Some(Sign::Capricorn),
		element: Element::Earth,
	};

	// 4 of Pentacles - Sun in Capricorn
	pub const FOUR_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::FOUR_OF_PENTACLES,
		planet: Some(Planet::Sun),
		sign: Some(Sign::Capricorn),
		element: Element::Earth,
	};

	// 5 of Pentacles - Mercury in Taurus
	pub const FIVE_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::FIVE_OF_PENTACLES,
		planet: Some(Planet::Mercury),
		sign: Some(Sign::Taurus),
		element: Element::Earth,
	};

	// 6 of Pentacles - Moon in Taurus
	pub const SIX_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::SIX_OF_PENTACLES,
		planet: Some(Planet::Moon),
		sign: Some(Sign::Taurus),
		element: Element::Earth,
	};

	// 7 of Pentacles - Saturn in Taurus
	pub const SEVEN_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::SEVEN_OF_PENTACLES,
		planet: Some(Planet::Saturn),
		sign: Some(Sign::Taurus),
		element: Element::Earth,
	};

	// 8 of Pentacles - Sun in Virgo
	pub const EIGHT_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::EIGHT_OF_PENTACLES,
		planet: Some(Planet::Sun),
		sign: Some(Sign::Virgo),
		element: Element::Earth,
	};

	// 9 of Pentacles - Venus in Virgo
	pub const NINE_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::NINE_OF_PENTACLES,
		planet: Some(Planet::Venus),
		sign: Some(Sign::Virgo),
		element: Element::Earth,
	};

	// 10 of Pentacles - Mercury in Virgo
	pub const TEN_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TEN_OF_PENTACLES,
		planet: Some(Planet::Mercury),
		sign: Some(Sign::Virgo),
		element: Element::Earth,
	};
	pub const PAGE_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::PAGE_OF_PENTACLES,
		planet: None,
		sign: None,
		element: Element::Earth,
	};
	pub const KNIGHT_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::KNIGHT_OF_PENTACLES,
		planet: None,
		sign: None,
		element: Element::Earth,
	};
	pub const QUEEN_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::QUEEN_OF_PENTACLES,
		planet: None,
		sign: None,
		element: Element::Earth,
	};
	pub const KING_OF_PENTACLES: TarotCardMeta = TarotCardMeta {
		card: TarotCard::KING_OF_PENTACLES,
		planet: None,
		sign: None,
		element: Element::Earth,
	};
}
