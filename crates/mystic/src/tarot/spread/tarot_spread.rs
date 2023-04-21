use super::super::*;


pub trait TarotSpread
where
	Self: Sized + std::fmt::Display,
{
	fn new() -> Self { Self::with_deck(&mut TarotDeck::shuffled()) }
	fn with_deck(deck: &mut TarotDeck) -> Self;
}


pub struct PastPresentFuture {
	pub card1: TarotCard,
	pub card2: TarotCard,
	pub card3: TarotCard,
}
impl TarotSpread for PastPresentFuture {
	fn with_deck(deck: &mut TarotDeck) -> Self {
		Self {
			card1: deck.pop().unwrap(),
			card2: deck.pop().unwrap(),
			card3: deck.pop().unwrap(),
		}
	}
}


impl std::fmt::Display for PastPresentFuture {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(
			format!(
				r#"
This is a three card spread, representing past, present and future.
The first card is: {}
The second card is: {}
The third card is: {}
	"#,
				self.card1, self.card2, self.card3
			)
			.as_str(),
		)
	}
}
