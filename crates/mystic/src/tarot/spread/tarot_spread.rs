use super::super::*;
use super::*;


pub trait TarotSpread {
	fn new(deck: &mut TarotDeck) -> Self;

	fn interpret(&self) -> String;
}


pub struct PastPresentFuture {
	pub card1: TarotCard,
	pub card2: TarotCard,
	pub card3: TarotCard,
}
impl TarotSpread for PastPresentFuture {
	fn new(deck: &mut TarotDeck) -> Self {
		Self {
			card1: deck.pop().unwrap(),
			card2: deck.pop().unwrap(),
			card3: deck.pop().unwrap(),
		}
	}

	fn interpret(&self) -> String {
		format!(
			r#"
This is a three card spread, representing past, present and future.
The first card is: {}
The second card is: {}
The third card is: {}

"#,
			self.card1, self.card2, self.card3
		)
	}
}
