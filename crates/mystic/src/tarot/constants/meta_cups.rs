use super::super::*;
use crate::astro::{
	chart::{Element, Sign},
	planets::Planet,
};

impl TarotCardMeta {
	// Ace of Cups - Cancer
	pub const ACE_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::ACE_OF_CUPS,
		planet: None,
		sign: Some(Sign::Cancer),
		element: Element::Water,
	};

	// 2 of Cups - Venus in Cancer
	pub const TWO_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TWO_OF_CUPS,
		planet: Some(Planet::Venus),
		sign: Some(Sign::Cancer),
		element: Element::Water,
	};

	// 3 of Cups - Mercury in Cancer
	pub const THREE_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THREE_OF_CUPS,
		planet: Some(Planet::Mercury),
		sign: Some(Sign::Cancer),
		element: Element::Water,
	};

	// 4 of Cups - Moon in Cancer
	pub const FOUR_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::FOUR_OF_CUPS,
		planet: Some(Planet::Moon),
		sign: Some(Sign::Cancer),
		element: Element::Water,
	};

	// 5 of Cups - Mars in Scorpio
	pub const FIVE_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::FIVE_OF_CUPS,
		planet: Some(Planet::Mars),
		sign: Some(Sign::Scorpio),
		element: Element::Water,
	};

	// 6 of Cups - Sun in Scorpio
	pub const SIX_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::SIX_OF_CUPS,
		planet: Some(Planet::Sun),
		sign: Some(Sign::Scorpio),
		element: Element::Water,
	};

	// 7 of Cups - Venus in Scorpio
	pub const SEVEN_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::SEVEN_OF_CUPS,
		planet: Some(Planet::Venus),
		sign: Some(Sign::Scorpio),
		element: Element::Water,
	};

	// 8 of Cups - Saturn in Pisces
	pub const EIGHT_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::EIGHT_OF_CUPS,
		planet: Some(Planet::Saturn),
		sign: Some(Sign::Pisces),
		element: Element::Water,
	};

	// 9 of Cups - Jupiter in Pisces
	pub const NINE_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::NINE_OF_CUPS,
		planet: Some(Planet::Jupiter),
		sign: Some(Sign::Pisces),
		element: Element::Water,
	};

	// 10 of Cups - Mars in Pisces
	pub const TEN_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TEN_OF_CUPS,
		planet: Some(Planet::Mars),
		sign: Some(Sign::Pisces),
		element: Element::Water,
	};
	pub const PAGE_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::PAGE_OF_CUPS,
		planet: None,
		sign: None,
		element: Element::Water,
	};
	pub const KNIGHT_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::KNIGHT_OF_CUPS,
		planet: None,
		sign: None,
		element: Element::Water,
	};
	pub const QUEEN_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::QUEEN_OF_CUPS,
		planet: None,
		sign: None,
		element: Element::Water,
	};
	pub const KING_OF_CUPS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::KING_OF_CUPS,
		planet: None,
		sign: None,
		element: Element::Water,
	};
}
