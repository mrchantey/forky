use std::process::Command;

fn main() -> std::io::Result<()> {
	let output = Command::new("reg")
		.args(&[
			"add",
			"HKCU\\Control Panel\\Mouse",
			"/v",
			"SwapMouseButtons",
			"/t",
			"REG_SZ",
			"/d",
			"1",
			"/f",
		])
		.output()?;

	if !output.status.success() {
		eprintln!(
			"Failed to change mouse button setting: {}",
			String::from_utf8_lossy(&output.stderr)
		);
	}

	Ok(())
}
