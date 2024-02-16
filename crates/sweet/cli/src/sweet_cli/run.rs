use super::*;
use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use forky_fs::terminal;
use forky_fs::FsWatcher;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::Duration;
use tower_livereload::LiveReloadLayer;
// use tower_livereload::Reloader;

pub type AnyhowJoinHandle = JoinHandle<Result<(), Error>>;

impl SweetCli {
	#[tokio::main]
	pub async fn run(&self) -> Result<()> {
		if self.run_tests_mode.is_some() && !self.watch {
			self.run_once().await
		} else {
			// run in watch mode OR interactive
			self.run_forever().await
		}
	}

	async fn run_once(&self) -> Result<()> {
		terminal::print_forky();
		let server = self.server.clone();
		let _server = std::thread::spawn(move || -> Result<()> {
			server.serve_with_options(None)
		});
		let should_kill = || false;
		self.build_wasm(should_kill)?;
		let result = self.run_tests(should_kill).await?.unwrap();
		if result.did_fail() {
			Err(anyhow!("Tests failed"))
		} else {
			Ok(())
		}
	}

	async fn run_forever(&self) -> Result<()> {
		terminal::clear();
		terminal::print_forky();
		let server = self.server.clone();
		let livereload = LiveReloadLayer::new();
		let reload = livereload.reloader();
		let _server_handle = std::thread::spawn(move || -> Result<()> {
			println!(
				"Starting server at {}\n",
				server.address.to_string_pretty()
			);
			server.serve_with_reload(livereload)
		});
		// let _static_handle = self.watch_static_forever(reload.clone());


		let kill = Arc::new(Mutex::new(()));
		let kill2 = kill.clone();
		let should_kill = move || -> bool { kill2.try_lock().is_ok() };

		loop {
			let change_listener = self.get_change_listener(kill.clone());

			if let Err(err) = self.build_wasm(should_kill.clone()) {
				eprintln!("sweet cli - wasm-bingden command failed: {:?}", err);
			} else {
				reload.reload();
				if self.run_tests_mode.is_some() {
					let _ = self.run_tests(should_kill.clone()).await?;
				}
			}
			//only joins in watch mode, otherwise block forever
			change_listener.join().unwrap()?;
		}
	}

	// fn watch_static_forever(
	// 	&self,
	// 	reload: Reloader,
	// ) -> JoinHandle<Result<()>> {
	// 	let watch_str = format!("{}/**", &self.server.dir);
	// 	println!("\nwatching server dir: {}\n", &watch_str);
	// 	std::thread::spawn(move || -> Result<()> {
	// 		FsWatcher::default()
	// 			.with_run_on_start(false)
	// 			.with_quiet(true)
	// 			// .with_watch(&watch_str)
	// 			.watch(|_| {
	// 				std::thread::sleep(Duration::from_millis(100));
	// 				reload.reload();
	// 				println!("Server dir change detected, reloading");
	// 				Ok(())
	// 			})
	// 	})
	// }

	fn get_change_listener(&self, kill: Arc<Mutex<()>>) -> AnyhowJoinHandle {
		let kill2 = kill.clone();
		let listener = if self.watch {
			std::thread::spawn(move || -> Result<()> {
				let kill_lock = kill2.lock().unwrap();
				FsWatcher::default()
					.with_watch("**/*.rs")
					// .with_watch(&format!("{}/**", &self.server.dir))
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
					//TODO use condvar
					std::thread::sleep(Duration::from_secs(1))
				}
			})
		};
		//wait for lock
		while kill.try_lock().is_ok() {
			std::thread::sleep(Duration::from_millis(100))
		}
		listener
	}
}
