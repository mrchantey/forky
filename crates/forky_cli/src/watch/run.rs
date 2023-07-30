use anyhow::Result;
use std::process::Command;

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
pub fn spawn_command(command: &Vec<&str>) -> Result<()> {
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
