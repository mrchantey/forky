use anyhow::Result;
use extend::ext;
use std::process::Child;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum ChildProcessStatus {
	Killed,
	ExitSuccess,
	ExitFail(i32),
}


#[ext]
pub impl Child {
	/// Returns either when child completes or kill returns true
	fn wait_killable(
		mut self,
		kill: impl Fn() -> bool,
	) -> Result<ChildProcessStatus> {
		loop {
			if kill() {
				self.kill()?;
				return Ok(ChildProcessStatus::Killed);
			}
			match self.try_wait() {
				Ok(Some(status)) => match status.success() {
					true => {
						return Ok(ChildProcessStatus::ExitSuccess);
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
