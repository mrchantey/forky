use forky_cli::*;
use forky_fs::*;

fn main() -> anyhow::Result<()> {
	ForkyCli.run_with_cli_args()
}
