use super::*;
use crate::astro::{
	chart::{Element, Sign},
	planets::Planet,
};


pub struct TarotCardMeta {
	pub card: TarotCard,
	pub planet: Option<Planet>,
	pub sign: Option<Sign>,
	pub element: Element,
}

impl TarotCard {
	pub fn meta(&self) -> TarotCardMeta { self.clone().into() }
}

impl Into<TarotCard> for TarotCardMeta {
	fn into(self) -> TarotCard { self.card }
}
impl Into<TarotCardMeta> for TarotCard {
	#[rustfmt::skip]
	fn into(self) -> TarotCardMeta {
		match self {
			TarotCard::THE_FOOL => TarotCardMeta::THE_FOOL,
			TarotCard::THE_MAGICIAN => TarotCardMeta::THE_MAGICIAN,
			TarotCard::THE_HIGH_PRIESTESS => TarotCardMeta::THE_HIGH_PRIESTESS,
			TarotCard::THE_EMPRESS => TarotCardMeta::THE_EMPRESS,
			TarotCard::THE_EMPEROR => TarotCardMeta::THE_EMPEROR,
			TarotCard::THE_HIEROPHANT => TarotCardMeta::THE_HIEROPHANT,
			TarotCard::THE_LOVERS => TarotCardMeta::THE_LOVERS,
			TarotCard::THE_CHARIOT => TarotCardMeta::THE_CHARIOT,
			TarotCard::STRENGTH => TarotCardMeta::STRENGTH,
			TarotCard::THE_HERMIT => TarotCardMeta::THE_HERMIT,
			TarotCard::THE_WHEEL_OF_FORTUNE => TarotCardMeta::THE_WHEEL_OF_FORTUNE,
			TarotCard::JUSTICE => TarotCardMeta::JUSTICE,
			TarotCard::THE_HANGED_MAN => TarotCardMeta::THE_HANGED_MAN,
			TarotCard::DEATH => TarotCardMeta::DEATH,
			TarotCard::TEMPERANCE => TarotCardMeta::TEMPERANCE,
			TarotCard::THE_DEVIL => TarotCardMeta::THE_DEVIL,
			TarotCard::THE_TOWER => TarotCardMeta::THE_TOWER,
			TarotCard::THE_STAR => TarotCardMeta::THE_STAR,
			TarotCard::THE_MOON => TarotCardMeta::THE_MOON,
			TarotCard::THE_SUN => TarotCardMeta::THE_SUN,
			TarotCard::JUDGMENT => TarotCardMeta::JUDGMENT,
			TarotCard::THE_WORLD => TarotCardMeta::THE_WORLD,
			TarotCard::ACE_OF_WANDS => TarotCardMeta::ACE_OF_WANDS,
			TarotCard::TWO_OF_WANDS => TarotCardMeta::TWO_OF_WANDS,
			TarotCard::THREE_OF_WANDS => TarotCardMeta::THREE_OF_WANDS,
			TarotCard::FOUR_OF_WANDS => TarotCardMeta::FOUR_OF_WANDS,
			TarotCard::FIVE_OF_WANDS => TarotCardMeta::FIVE_OF_WANDS,
			TarotCard::SIX_OF_WANDS => TarotCardMeta::SIX_OF_WANDS,
			TarotCard::SEVEN_OF_WANDS => TarotCardMeta::SEVEN_OF_WANDS,
			TarotCard::EIGHT_OF_WANDS => TarotCardMeta::EIGHT_OF_WANDS,
			TarotCard::NINE_OF_WANDS => TarotCardMeta::NINE_OF_WANDS,
			TarotCard::TEN_OF_WANDS => TarotCardMeta::TEN_OF_WANDS,
			TarotCard::PAGE_OF_WANDS => TarotCardMeta::PAGE_OF_WANDS,
			TarotCard::KNIGHT_OF_WANDS => TarotCardMeta::KNIGHT_OF_WANDS,
			TarotCard::QUEEN_OF_WANDS => TarotCardMeta::QUEEN_OF_WANDS,
			TarotCard::KING_OF_WANDS => TarotCardMeta::KING_OF_WANDS,
			TarotCard::ACE_OF_PENTACLES => TarotCardMeta::ACE_OF_PENTACLES,
			TarotCard::TWO_OF_PENTACLES => TarotCardMeta::TWO_OF_PENTACLES,
			TarotCard::THREE_OF_PENTACLES => TarotCardMeta::THREE_OF_PENTACLES,
			TarotCard::FOUR_OF_PENTACLES => TarotCardMeta::FOUR_OF_PENTACLES,
			TarotCard::FIVE_OF_PENTACLES => TarotCardMeta::FIVE_OF_PENTACLES,
			TarotCard::SIX_OF_PENTACLES => TarotCardMeta::SIX_OF_PENTACLES,
			TarotCard::SEVEN_OF_PENTACLES => TarotCardMeta::SEVEN_OF_PENTACLES,
			TarotCard::EIGHT_OF_PENTACLES => TarotCardMeta::EIGHT_OF_PENTACLES,
			TarotCard::NINE_OF_PENTACLES => TarotCardMeta::NINE_OF_PENTACLES,
			TarotCard::TEN_OF_PENTACLES => TarotCardMeta::TEN_OF_PENTACLES,
			TarotCard::PAGE_OF_PENTACLES => TarotCardMeta::PAGE_OF_PENTACLES,
			TarotCard::KNIGHT_OF_PENTACLES => TarotCardMeta::KNIGHT_OF_PENTACLES,
			TarotCard::QUEEN_OF_PENTACLES => TarotCardMeta::QUEEN_OF_PENTACLES,
			TarotCard::KING_OF_PENTACLES => TarotCardMeta::KING_OF_PENTACLES,
			TarotCard::ACE_OF_SWORDS => TarotCardMeta::ACE_OF_SWORDS,
			TarotCard::TWO_OF_SWORDS => TarotCardMeta::TWO_OF_SWORDS,
			TarotCard::THREE_OF_SWORDS => TarotCardMeta::THREE_OF_SWORDS,
			TarotCard::FOUR_OF_SWORDS => TarotCardMeta::FOUR_OF_SWORDS,
			TarotCard::FIVE_OF_SWORDS => TarotCardMeta::FIVE_OF_SWORDS,
			TarotCard::SIX_OF_SWORDS => TarotCardMeta::SIX_OF_SWORDS,
			TarotCard::SEVEN_OF_SWORDS => TarotCardMeta::SEVEN_OF_SWORDS,
			TarotCard::EIGHT_OF_SWORDS => TarotCardMeta::EIGHT_OF_SWORDS,
			TarotCard::NINE_OF_SWORDS => TarotCardMeta::NINE_OF_SWORDS,
			TarotCard::TEN_OF_SWORDS => TarotCardMeta::TEN_OF_SWORDS,
			TarotCard::PAGE_OF_SWORDS => TarotCardMeta::PAGE_OF_SWORDS,
			TarotCard::KNIGHT_OF_SWORDS => TarotCardMeta::KNIGHT_OF_SWORDS,
			TarotCard::QUEEN_OF_SWORDS => TarotCardMeta::QUEEN_OF_SWORDS,
			TarotCard::KING_OF_SWORDS => TarotCardMeta::KING_OF_SWORDS,
			TarotCard::ACE_OF_CUPS => TarotCardMeta::ACE_OF_CUPS,
			TarotCard::TWO_OF_CUPS => TarotCardMeta::TWO_OF_CUPS,
			TarotCard::THREE_OF_CUPS => TarotCardMeta::THREE_OF_CUPS,
			TarotCard::FOUR_OF_CUPS => TarotCardMeta::FOUR_OF_CUPS,
			TarotCard::FIVE_OF_CUPS => TarotCardMeta::FIVE_OF_CUPS,
			TarotCard::SIX_OF_CUPS => TarotCardMeta::SIX_OF_CUPS,
			TarotCard::SEVEN_OF_CUPS => TarotCardMeta::SEVEN_OF_CUPS,
			TarotCard::EIGHT_OF_CUPS => TarotCardMeta::EIGHT_OF_CUPS,
			TarotCard::NINE_OF_CUPS => TarotCardMeta::NINE_OF_CUPS,
			TarotCard::TEN_OF_CUPS => TarotCardMeta::TEN_OF_CUPS,
			TarotCard::PAGE_OF_CUPS => TarotCardMeta::PAGE_OF_CUPS,
			TarotCard::KNIGHT_OF_CUPS => TarotCardMeta::KNIGHT_OF_CUPS,
			TarotCard::QUEEN_OF_CUPS => TarotCardMeta::QUEEN_OF_CUPS,
			TarotCard::KING_OF_CUPS => TarotCardMeta::KING_OF_CUPS,
		}
	}
}
