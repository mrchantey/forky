use anyhow::Result;
use extend::ext;
use std::process::Child;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub enum Status {
	Killed,
	ExitSuccess,
	ExitFail(i32),
}


#[ext]
pub impl Child {
	/// Returns either when child completes or mutex is unlocked
	fn wait_killable(mut self, kill: impl Fn() -> bool) -> Result<()> {
		loop {
			if kill() {
				let _ = self.kill();
				return Ok(());
			}
			match self.try_wait() {
				Ok(Some(status)) => match status.success() {
					true => return Ok(()),
					false => anyhow::bail!(
						"Child process exited with error code: {}",
						status.code().unwrap_or(-1)
					),
				},
				Ok(None) => {
					//is this required?
					thread::sleep(Duration::from_millis(10));
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
