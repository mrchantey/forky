use anyhow::Result;
use mystic::{self, cli::Subcommand};

#[rustfmt::skip]
fn main() -> Result<()> { 
	mystic::cli::MysticCommand.run_with_cli_args()
}
