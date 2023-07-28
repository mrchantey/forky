use anyhow::Result;
use clap::Arg;
use clap::ArgAction;
use forky_core::StringX;
use forky_fs::FsWatcher;
// use forky_fs::FsWatcher;
use forky_fs::Subcommand;
use std::process::Command;

pub struct WatchCommand;

impl Subcommand for WatchCommand {
	fn name(&self) -> &'static str { "watch" }
	fn about(&self) -> &'static str { "execute command on file change" }

	fn append_command(&self, command: clap::Command) -> clap::Command {
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
		FsWatcher::default()
			.with_watches(watches)
			.with_ignores(ignores)
			.watch(|_| run(&cmd))
	}
}

/// Run a command and print the output. Returns error only if execution failed, not if it returns error
pub fn run(command: &Vec<&str>) -> Result<()> {
	let output = if cfg!(target_os = "windows") {
		// command.insert(0, &"\\C");
		// Command::new("cmd")
		// command.insert(0, &"-Command");
		Command::new("powershell")
			.arg("-Command")
			.args(command)
			.output()
	} else {
		// command.insert(0, &"-c");
		Command::new("sh").arg("-c").args(command).output()
	}?;
	if output.status.success() {
		let stdout = String::from_utf8_lossy(&output.stdout);
		println!("{}", stdout);
	} else {
		let stderr = String::from_utf8_lossy(&output.stderr);
		eprintln!("{}", stderr);
	}
	Ok(())
}
