use anyhow::Result;
use clap::ArgAction;
use clap::Parser;
use forky_fs::prelude::*;

#[derive(Parser)]
#[command(name = "watch", about = "execute command on file change")]
pub struct WatchCommand {
	/// the space separated command to run, ie 'forky watch -- echo howdy'
	#[arg(required = true, action = ArgAction::Append)]
	cmd: Vec<String>,

	/// paths to watch
	#[arg(short = 'w', long = "watch", action = ArgAction::Append)]
	watch: Option<Vec<String>>,

	/// paths to ignore
	#[arg(short = 'i', long = "ignore", action = ArgAction::Append)]
	ignore: Option<Vec<String>>,

	/// only run once instead of watching indefinitely
	#[arg(long = "once")]
	once: bool,

	/// Use rusty defaults for watch and ignore
	#[arg(long)]
	rusty: bool,
}

impl WatchCommand {
	pub fn run(&self) -> Result<()> {
		let cmd = self.cmd.iter().map(|s| s.as_str()).collect::<Vec<_>>();
		let mut watches = self
			.watch
			.as_ref()
			.map(|w| w.iter().map(|s| s.as_str()).collect::<Vec<_>>())
			.unwrap_or_default();
		let mut ignores = self
			.ignore
			.as_ref()
			.map(|i| i.iter().map(|s| s.as_str()).collect::<Vec<_>>())
			.unwrap_or_default();
		self.try_append_rusty(&mut watches, &mut ignores);

		let watcher = FsWatcher::default()
			.with_watches(watches)
			.with_ignores(ignores);

		if self.once {
			watcher.block()?;
			CommandExt::spawn_command_blocking(&cmd)
		} else {
			watcher.watch(|_| CommandExt::spawn_command_blocking(&cmd))
		}
	}

	fn try_append_rusty(
		&self,
		watches: &mut Vec<&str>,
		ignores: &mut Vec<&str>,
	) {
		if !self.rusty {
			return;
		}
		watches.push("**/*.rs");
		watches.push("**/*.ts");
		ignores.push("{.git,target,html}/**");
		// ignores.push("**/mod.rs");
	}
}
