use forky_ai::ChatGptInstance;
use forky_ai::Llm;
use mystic::tarot::*;


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


impl TarotSpread {
	fn print(&self) -> String;

	async fn interpret(&self, interpreter: &impl Llm) -> Result<String> {
		interpreter
			.message_stream_print(
				create_llm_message(self.print().as_str()).as_str(),
			)
			.await
	}
}
