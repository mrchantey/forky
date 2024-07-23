use anyhow::Result;
use extend::ext;
use std::process::Child;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum ChildProcessStatus {
	Killed,
	ExitSuccess(i32),
	ExitFail(i32),
}

#[ext]
pub impl Child {
	/// Returns either when child completes or kill returns true
	fn wait_killable(
		mut self,
		should_kill: impl Fn() -> bool,
	) -> Result<ChildProcessStatus> {
		loop {
			if should_kill() {
				self.kill()?;
				return Ok(ChildProcessStatus::Killed);
			}
			match self.try_wait() {
				Ok(Some(status)) => match status.success() {
					true => {
						// println!("output: {:?}", self.wait_with_output());
						return Ok(ChildProcessStatus::ExitSuccess(0));
					}
					false => {
						return Ok(ChildProcessStatus::ExitFail(
							status.code().unwrap_or(-1),
						))
					}
				},
				Ok(None) => {
					//still running, is sleep required?
					thread::sleep(Duration::from_millis(1));
					continue;
				}
				Err(err) => {
					anyhow::bail!(
						"Error waiting for the child process: {}",
						err
					)
				}
			}
		}
	}
}
