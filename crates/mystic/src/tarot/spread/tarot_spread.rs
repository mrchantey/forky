use super::super::*;
use crate::*;
use anyhow::Result;
use forky_ai::Llm;
// use super::*;




pub trait TarotSpread {
	fn new(deck: &mut TarotDeck) -> Self;

	fn print(&self) -> String;

	async fn interpret(&self, interpreter: &impl Llm) -> Result<String> {
		interpreter
			.message_stream_print(
				create_llm_message(self.print().as_str()).as_str(),
			)
			.await
	}
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

	fn print(&self) -> String {
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
