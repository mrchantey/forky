use super::*;
use anyhow::Result;
use clap::Arg;
use clap::ArgAction;
use forky_fs::Subcommand;

pub struct ServerCommand;

impl Subcommand for ServerCommand {
	fn name(&self) -> &'static str { "serve" }
	fn about(&self) -> &'static str { "serve static files" }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command.arg(Arg::new("dir").required(false).action(ArgAction::Append))
		// .arg(
		// 	Arg::new("watch")
		// 		.required(false)
		// 		.short('w')
		// 		.long("watch")
		// 		.action(ArgAction::Append),
		// )
	}

	fn run(&self, args: &clap::ArgMatches) -> Result<()> {
		// let dir = *args.get_one::<&str>("dir").unwrap_or(&"html");
		// let watches = args
		// 	.get_many::<String>("watch")
		// 	.unwrap_or_default()
		// 	.map(|s| s.to_str())
		// 	.collect::<Vec<_>>();
		// ServerConfig::
		// let server = Server::default();
		Server::serve_forever(|| Server::default())
		// FsWatcher::default()
		// 	// .with_dont_clear()
		// 	.with_watches(watches)
		// 	.with_ignores(ignores)
		// 	.watch(|_| run(&cmd))
	}
}
