use anyhow::Result;
use std::process::Child;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

/// Run a command and pipe the output to stdio. Returns error only if execution failed, not if it returns error
pub fn spawn_command(args: &Vec<&str>) -> Result<Child> {
	let child = get_command(args)
		// .stdout(Stdio::piped())
		.spawn()?;
	Ok(child)
}

pub fn spawn_command_blocking(args: &Vec<&str>) -> Result<()> {
	let _ = get_command(args)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
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

pub fn spawn_command_with_shell_blocking(args: &Vec<&str>) -> Result<()> {
	let _ = get_command_with_shell(args)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.output()?;
	Ok(())
}

fn get_command_with_shell(args: &Vec<&str>) -> Command {
	let is_windows = cfg!(target_os = "windows");
	let (cmd, arg) = if is_windows {
		// ("cmd", "\\C")
		("powershell", "-Command")
	} else {
		("sh", "-c")
	};
	let mut cmd = Command::new(cmd);
	cmd.arg(arg);
	cmd.args(args);
	cmd
}

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
