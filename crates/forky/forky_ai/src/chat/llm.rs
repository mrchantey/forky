use anyhow::Result;
use std::io::stdout;
use std::io::Write;

//

pub trait Llm {
	async fn message(&self, message: &str) -> Result<String>;
	async fn message_stream<F>(
		&self,
		message: &str,
		on_data: F,
	) -> Result<String>
	where
		F: Fn(&str);
	async fn message_stream_print(&self, message: &str) -> Result<String> {
		self.message_stream(message, |each| {
			print!("{}", each);
			stdout().lock().flush().unwrap();
		})
		.await
	}
}
