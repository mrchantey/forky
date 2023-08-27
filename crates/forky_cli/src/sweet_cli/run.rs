use super::*;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;
use forky_fs::terminal;
use forky_fs::FsWatcher;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::Duration;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;

pub type AnyhowJoinHandle = JoinHandle<Result<(), Error>>;

impl SweetCli {
	#[tokio::main]
	pub async fn run(&self) -> Result<()> {
		terminal::clear();
		terminal::print_forky();

		let (_server, reload) = self.get_server()?;

		if let Some(mode) = self.run_tests && !self.watch {
				let should_kill = || false;
				self.build_wasm(should_kill)?;
				let result = self.run_tests(mode, should_kill).await?.unwrap();
				if result.cases.failed > 0 {
					bail!("Tests failed");
				}
		}else{

			let kill = Arc::new(Mutex::new(()));
			let kill2 = kill.clone();
			let should_kill = move || -> bool { kill2.try_lock().is_ok() };
	
			loop {
				let change_listener = self.get_change_listener(kill.clone());
	
				if let Err(err) = self.build_wasm(should_kill.clone()) {
					eprintln!("sweet cli - build failed: {:?}", err);
				} else {
					if let Some(reload) = reload.as_ref() {
						reload.reload();
					}
					if let Some(mode) = self.run_tests {
						let _ = self.run_tests(mode, should_kill.clone()).await?;
					}
				}
				change_listener.join().unwrap()?;
			}
		}


		Ok(())
	}

	fn get_server(&self) -> Result<(AnyhowJoinHandle, Option<Reloader>)> {
		let server = self.server.clone();
		if None == self.run_tests || !self.watch {
			let server_handle = std::thread::spawn(move || -> Result<()> {
				server.serve_with_options(None)
			});
			Ok((server_handle, None))
		} else {
			let livereload = LiveReloadLayer::new();
			let reload = livereload.reloader();
			let server_handle = std::thread::spawn(move || -> Result<()> {
				println!(
					"Starting server at {}\n",
					server.address.to_string_pretty()
				);
				server.serve_with_reload(livereload)
			});
			Ok((server_handle, Some(reload)))
		}
	}

	fn get_change_listener(&self, kill: Arc<Mutex<()>>) -> AnyhowJoinHandle {
		let kill2 = kill.clone();
		let listener = if self.watch {
			std::thread::spawn(move || -> Result<()> {
				let kill_lock = kill2.lock().unwrap();
				FsWatcher::default()
					.with_watch("**/*.rs")
					.with_ignore("**/target/**")
					.block()?;
				drop(kill_lock);
				Ok(())
			})
		} else {
			// otherwise lock mutex and loop forever
			std::thread::spawn(move || -> Result<()> {
				let _kill_lock = kill2.lock().unwrap();
				loop {
					std::thread::sleep(Duration::from_secs(1))
				}
			})
		};
		//wait for fswatcher to spin up
		while kill.try_lock().is_ok() {
			std::thread::sleep(Duration::from_millis(1))
		}
		listener
	}
}
