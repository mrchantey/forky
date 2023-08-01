use anyhow::Result;
use forky_core::GenericVecExt;
use forky_core::OptionTExt;
use futures::lock::Mutex;
use std::process::Child;
use std::process::Command;
use std::process::Output;

/// Run a command and pipe the output to stdio. Returns error only if execution failed, not if it returns error
pub fn spawn_command(args: &Vec<&str>) -> Result<Child> {
	let child = get_command(args).spawn()?;
	Ok(child)
}

pub fn spawn_command_blocking(args: &Vec<&str>) -> Result<()> {
	let _ = get_command(args)
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.output()?;
	Ok(())
}

pub fn spawn_command_hold_stdio(args: &Vec<&str>) -> Result<CommandOutput> {
	let out = get_command(args).output()?;
	Ok(out.into())
}

fn get_command(args: &Vec<&str>) -> Command {
	let mut cmd = Command::new(args[0]);
	cmd.args(args[1..].iter());
	cmd
}
// fn get_command(command: &Vec<&str>) -> Command {
// 	let is_windows = cfg!(target_os = "windows");
// 	let (cmd, arg) = if is_windows {
// 		// ("cmd", "\\C")
// 		("powershell", "-Command")
// 	} else {
// 		("sh", "-c")
// 	};
// 	let mut cmd = Command::new(cmd);
// 	cmd.arg(arg);
// 	cmd
// }

pub struct CommandOutput {
	pub success: bool,
	pub stdout: String,
	pub stderr: String,
}

impl From<Output> for CommandOutput {
	fn from(output: Output) -> Self {
		let stdout = String::from_utf8_lossy(&output.stdout).to_string();
		let stderr = String::from_utf8_lossy(&output.stderr).to_string();
		CommandOutput {
			success: output.status.success(),
			stdout,
			stderr,
		}
	}
}
