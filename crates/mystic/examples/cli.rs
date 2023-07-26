use anyhow::Result;
use forky_fs::Subcommand;
use mystic::cli::MysticCommand;

#[rustfmt::skip]
fn main() -> Result<()> { 
	MysticCommand.run_with_cli_args()
}
