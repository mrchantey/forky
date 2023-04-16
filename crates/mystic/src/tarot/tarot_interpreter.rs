use crate::tarot::spread::TarotSpread;
use super::*;
use anyhow::Result;
use forky_ai::ChatGptInstance;

pub async fn run_tarot_interpreter() -> Result<()> {
	let mut deck = TarotDeck::new();
	deck.shuffle();
	let spread = spread::PastPresentFuture::new(&mut deck);

	let gpt = ChatGptInstance::new()?;
	println!("interpreting your spread...\n{}", spread.print());
	let _result = spread.interpret(&gpt).await?;
	// println!("{}", result);
	Ok(())
}
