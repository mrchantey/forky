use anyhow::Result;
use mystic;
#[tokio::main]
#[rustfmt::skip]
async fn main() -> Result<()> { 
	mystic::cli::run().await 
}