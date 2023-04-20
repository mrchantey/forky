use super::super::*;
use crate::astro::{
	chart::{Element, Sign},
	planets::Planet,
};

impl TarotCardMeta {
	// The Fool - Uranus
	pub const THE_FOOL: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_FOOL,
		planet: Some(Planet::Uranus),
		sign: None,
		element: Element::Air,
	};

	// The Magician - Mercury
	pub const THE_MAGICIAN: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_MAGICIAN,
		planet: Some(Planet::Mercury),
		sign: None,
		element: Element::Air,
	};

	// The High Priestess - Moon
	pub const THE_HIGH_PRIESTESS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_HIGH_PRIESTESS,
		planet: Some(Planet::Moon),
		sign: None,
		element: Element::Water,
	};

	// The Empress - Venus
	pub const THE_EMPRESS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_EMPRESS,
		planet: Some(Planet::Venus),
		sign: None,
		element: Element::Earth,
	};

	// The Emperor - Aries
	pub const THE_EMPEROR: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_EMPEROR,
		planet: None,
		sign: Some(Sign::Aries),
		element: Element::Fire,
	};


	// The Hierophant - Taurus
	pub const THE_HIEROPHANT: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_HIEROPHANT,
		planet: None,
		sign: Some(Sign::Taurus),
		element: Element::Earth,
	};

	// The Lovers - Gemini
	pub const THE_LOVERS: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_LOVERS,
		planet: None,
		sign: Some(Sign::Gemini),
		element: Element::Air,
	};

	// The Chariot - Cancer
	pub const THE_CHARIOT: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_CHARIOT,
		planet: None,
		sign: Some(Sign::Cancer),
		element: Element::Water,
	};

	// Strength - Leo
	pub const STRENGTH: TarotCardMeta = TarotCardMeta {
		card: TarotCard::STRENGTH,
		planet: None,
		sign: Some(Sign::Leo),
		element: Element::Fire,
	};

	// The Hermit - Virgo
	pub const THE_HERMIT: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_HERMIT,
		planet: None,
		sign: Some(Sign::Virgo),
		element: Element::Earth,
	};

	// The Wheel of Fortune - Jupiter
	pub const THE_WHEEL_OF_FORTUNE: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_WHEEL_OF_FORTUNE,
		planet: Some(Planet::Jupiter),
		sign: None,
		element: Element::Air,
	};

	// Justice - Libra
	pub const JUSTICE: TarotCardMeta = TarotCardMeta {
		card: TarotCard::JUSTICE,
		planet: None,
		sign: Some(Sign::Libra),
		element: Element::Air,
	};

	// The Hanged Man - Neptune
	pub const THE_HANGED_MAN: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_HANGED_MAN,
		planet: Some(Planet::Neptune),
		sign: None,
		element: Element::Water,
	};

	// Death - Scorpio
	pub const DEATH: TarotCardMeta = TarotCardMeta {
		card: TarotCard::DEATH,
		planet: None,
		sign: Some(Sign::Scorpio),
		element: Element::Water,
	};

	// Temperance - Sagittarius
	pub const TEMPERANCE: TarotCardMeta = TarotCardMeta {
		card: TarotCard::TEMPERANCE,
		planet: None,
		sign: Some(Sign::Sagittarius),
		element: Element::Fire,
	};

	// The Devil - Capricorn
	pub const THE_DEVIL: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_DEVIL,
		planet: None,
		sign: Some(Sign::Capricorn),
		element: Element::Earth,
	};

	// The Tower - Mars
	pub const THE_TOWER: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_TOWER,
		planet: Some(Planet::Mars),
		sign: None,
		element: Element::Fire,
	};

	// The Star - Aquarius
	pub const THE_STAR: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_STAR,
		planet: None,
		sign: Some(Sign::Aquarius),
		element: Element::Air,
	};

	// The Moon - Pisces
	pub const THE_MOON: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_MOON,
		planet: None,
		sign: Some(Sign::Pisces),
		element: Element::Water,
	};

	// The Sun - Sun
	pub const THE_SUN: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_SUN,
		planet: Some(Planet::Sun),
		sign: None,
		element: Element::Fire,
	};

	// Judgment - Pluto
	pub const JUDGMENT: TarotCardMeta = TarotCardMeta {
		card: TarotCard::JUDGMENT,
		planet: Some(Planet::Pluto),
		sign: None,
		element: Element::Water,
	};

	// The World - Saturn
	pub const THE_WORLD: TarotCardMeta = TarotCardMeta {
		card: TarotCard::THE_WORLD,
		planet: Some(Planet::Saturn),
		sign: None,
		element: Element::Earth,
	};
}
