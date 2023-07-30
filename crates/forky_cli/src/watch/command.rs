use super::*;
use anyhow::Result;
use clap::Arg;
use clap::ArgAction;
use clap::Command;
use forky_core::StringX;
use forky_fs::FsWatcher;
use forky_fs::Subcommand;

pub struct WatchCommand;

impl Subcommand for WatchCommand {
	fn name(&self) -> &'static str { "watch" }
	fn about(&self) -> &'static str { "execute command on file change" }

	fn append_command(&self, command: Command) -> Command {
		command
			.arg(Arg::new("cmd").required(true).action(ArgAction::Append))
			.arg(
				Arg::new("watch")
					.required(false)
					.short('w')
					.long("watch")
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("ignore")
					.required(false)
					.short('i')
					.long("ignore")
					.action(ArgAction::Append),
			)
	}

	fn run(&self, args: &clap::ArgMatches) -> Result<()> {
		let cmd = args
			.get_many::<String>("cmd")
			.unwrap()
			.map(|s| s.to_str())
			.collect::<Vec<_>>();
		let watches = args
			.get_many::<String>("watch")
			.unwrap_or_default()
			.map(|s| s.to_str())
			.collect::<Vec<_>>();
		let ignores = args
			.get_many::<String>("ignore")
			.unwrap_or_default()
			.map(|s| s.to_str())
			.collect::<Vec<_>>();

		// println!("watch\ncommand:{:?}", cmd);
		FsWatcher::default()
			// .with_dont_clear()
			.with_watches(watches)
			.with_ignores(ignores)
			.watch(|_| spawn_command(&cmd))
	}
}
