use anyhow::Result;
use clap::Arg;
use clap::ArgAction;
use clap::Command;
use colorize::*;
use forky_core::StringX;
use forky_fs::process::spawn_command_blocking;
use forky_fs::FsWatcher;
use forky_fs::Subcommand;

pub struct WatchCommand;

impl Subcommand for WatchCommand {
	fn name(&self) -> &'static str { "watch" }
	fn about(&self) -> &'static str { "execute command on file change" }

	fn append_command(&self, command: Command) -> Command {
		// let cmd

		command
			.arg(
				Arg::new("cmd")
					.help(format!(
						"the space seperated command to run, ie {}",
						"forky watch echo howdy".green().bold()
					))
					.required(true)
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("watch")
					.help("paths to watch")
					.required(false)
					.short('w')
					.long("watch")
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("ignore")
					.help("paths to ignore")
					.required(false)
					.short('i')
					.long("ignore")
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("once")
					.help("only run once instead of watching indefinitely")
					.required(false)
					.short('b')
					.long("once")
					.action(ArgAction::SetTrue),
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

		let watcher = FsWatcher::default()
			.with_watches(watches)
			.with_ignores(ignores);

		if args.get_flag("once") {
			watcher.block()?;
			// futures::executor::block_on(watcher.block_async())?;
			spawn_command_blocking(&cmd)
		} else {
			// futures::executor::block_on(watcher.watch_async(|_| spawn_command(&cmd)))
			watcher.watch(|_| spawn_command_blocking(&cmd))
		}
	}
}
