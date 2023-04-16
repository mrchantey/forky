use anyhow::Result;
use chatgpt::prelude::*;
use dotenv::dotenv;
use std::env;


const CHATGPT_KEY: &str = "CHATGPT_KEY";


fn init() -> Result<ChatGPT> {
	dotenv().ok();

	let key = env::var(CHATGPT_KEY)
		.expect(&format!("no key {CHATGPT_KEY} provided")[..]);

	let client = ChatGPT::new_with_config(
		key,
		ModelConfiguration {
			// engine: ChatGPTEngine::Gpt4, //not yet available
			..Default::default()
		},
	)?;

	Ok(client)
}


pub async fn hello_world() -> Result<()> {
	let client = init()?;

	let response = client
		.send_message("Describe in five words the Rust programming language.")
		.await?;

	println!("Response: {}", response.message().content);

	Ok(())
}


pub async fn hello_conversation() -> Result<()> {
	let client = init()?;

	let mut conversation: Conversation = client.new_conversation();

	let _response_a = conversation
		.send_message(
			"Could you describe the Rust programming language in 5 words?",
		)
		.await?;
	let _response_b = conversation
		.send_message("Now could you do the same, but for Kotlin?")
		.await?;

	// You can also access the message history itself
	for message in &conversation.history {
		println!("{message:#?}")
	}

	Ok(())
}
