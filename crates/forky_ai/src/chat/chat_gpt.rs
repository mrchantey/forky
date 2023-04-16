use super::*;
use anyhow::Result;
use chatgpt::prelude::*;
use dotenv::dotenv;
use futures_util::StreamExt;
use std::env;


const CHATGPT_KEY: &str = "CHATGPT_KEY";




pub struct ChatGptInstance {
	client: ChatGPT,
}




impl ChatGptInstance {
	pub fn new() -> Result<Self> {
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

		Ok(Self { client })
	}
}


impl Llm for ChatGptInstance {
	async fn message(&self, message: &str) -> Result<String> {
		let response = self.client.send_message(message).await?;

		Ok(response.message().content.clone())
	}


	async fn message_stream<F>(
		&self,
		message: &str,
		on_data: F,
	) -> Result<String>
	where
		F: Fn(&str),
	{
		let stream = self.client.send_message_streaming(message).await?;
		// let mut response = ;
		let (response, _) = stream
			.fold(
				(String::new(), on_data),
				|(mut response, on_data), each| async move {
					match each {
						ResponseChunk::Content {
							delta,
							response_index: _,
						} => {
							on_data(&delta);
							response.push_str(&delta);
						}
						_ => {}
					}
					(response, on_data)
				},
			)
			.await;
		Ok(response)
	}
}
