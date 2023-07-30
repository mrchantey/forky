use forky_cli::*;
use forky_fs::*;

async fn main() -> anyhow::Result<()> { common::ForkyCli.run_with_cli_args() }
