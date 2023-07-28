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

		// println!("watch\ncommand:{:?}", cmd);
		FsWatcher::default()
			// .with_dont_clear()
			.with_watches(watches)
			.with_ignores(ignores)
			.watch(|_| run(&cmd))
	}
}

fn get_command() -> Command {
	let is_windows = cfg!(target_os = "windows");
	let (cmd, arg) = if is_windows {
		// ("cmd", "\\C")
		("powershell", "-Command")
	} else {
		("sh", "-c")
	};
	let mut cmd = Command::new(cmd);
	cmd.arg(arg);
	cmd
}

/// Run a command and print the output. Returns error only if execution failed, not if it returns error
pub fn run(command: &Vec<&str>) -> Result<()> {
	let _ = get_command()
		.args(command)
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.output()?;
	// if output.status.success() {
	// let stdout = String::from_utf8_lossy(&output.stdout);
	// println!("{}", stdout);
	// } else {
	// let stderr = String::from_utf8_lossy(&output.stderr);
	// eprintln!("{}", stderr);
	// }
	Ok(())
}
