use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use forky_fs::utility::CommandExt;
use forky_fs::utility::FsExt;
use std::path::PathBuf;

/// Execute a command in each directory in the current directory
#[derive(Parser)]
#[command(name = "fed")]
pub struct ForEachDirectory {
	/// Command to execute in each directory
	#[arg()]
	command: Vec<String>,

	/// execute `git status` and error if any not clean
	#[arg(long)]
	git: bool,
}

impl ForEachDirectory {
	pub fn run(self) -> Result<()> {
		vec_cmd_for_each_dir(&self.command)?;
		self.git()?;
		Ok(())
	}
	fn git(&self) -> Result<()> {
		if !self.git {
			return Ok(());
		}
		for_each_dir(|path| {
			let mut cmd = CommandExt::from_whitespace("git status --porcelain");
			cmd.current_dir(path);
			CommandExt::unwrap_output_empty(cmd).map_err(|_| {
				anyhow::anyhow!("{}", "Unstaged changes".to_string().red())
			})
		})
	}
}

// fn whitespace_cmd_for_each(cmd: &str) -> Result<()> {
// 	for_each_dir(|path: &PathBuf| -> Result<()> {
// 		let mut cmd = CommandExt::from_whitespace(cmd);
// 		cmd.current_dir(path);
// 		CommandExt::unwrap_status(cmd)
// 	})
// }


fn vec_cmd_for_each_dir(cmd: &Vec<String>) -> Result<()> {
	if cmd.is_empty() {
		return Ok(());
	}
	for_each_dir(|path| {
		let mut cmd = CommandExt::from_vec(&cmd);
		cmd.current_dir(path);
		CommandExt::unwrap_status(cmd)
	})
}

fn for_each_dir(mut func: impl FnMut(&PathBuf) -> Result<()>) -> Result<()> {
	let current_dir = std::env::current_dir()?;
	let mut outcomes = Vec::<(PathBuf, Option<String>)>::new();

	// Get all entries in current directory
	for entry in FsExt::read_dir(&current_dir)? {
		let path = entry.path();

		if path.is_dir() {
			match func(&path) {
				Ok(_) => {
					outcomes.push((path, None));
				}
				Err(err) => {
					outcomes.push((path, Some(format!("{}", err))));
				}
			}
		}
	}

	// Report failures if any
	if !outcomes.is_empty() {
		outcomes.sort_by(|a, b| a.0.cmp(&b.0));
		for (dir, status) in outcomes.iter() {
			let status_str = if status.is_some() {
				" FAIL ".black().bold().redb()
			} else {
				" PASS ".black().bold().greenb()
			};
			let msg = status.as_deref().unwrap_or("");
			println!("{} {}\t{}", status_str, dir.display(), msg);
		}
	}
	if outcomes.iter().any(|(_, status)| status.is_some()) {
		Err(anyhow::anyhow!("Some directories failed"))
	} else {
		Ok(())
	}
}
