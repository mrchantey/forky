use anyhow::Result;

use super::*;

pub async fn hello_world() -> Result<()> {
	let instance = ChatGptInstance::new()?;
	let response = instance.message("Describe love in five words.").await?;

	println!("Response: {}", response);

	Ok(())
}


// pub async fn hello_conversation() -> Result<()> {
// 	let client = init()?;

// 	let mut conversation: Conversation = client.new_conversation();

// 	let _response_a = conversation
// 		.send_message(
// 			"Could you describe the Rust programming language in 5 words?",
// 		)
// 		.await?;
// 	let _response_b = conversation
// 		.send_message("Now could you do the same, but for Kotlin?")
// 		.await?;

// 	// You can also access the message history itself
// 	for message in &conversation.history {
// 		println!("{message:#?}")
// 	}

// 	Ok(())
// }
