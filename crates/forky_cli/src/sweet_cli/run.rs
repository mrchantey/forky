use super::*;
use anyhow::Error;
use anyhow::Result;
use forky_fs::terminal;
use forky_fs::FsWatcher;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::Duration;
use tower_livereload::LiveReloadLayer;

pub type ChangeListener = JoinHandle<Result<(), Error>>;

impl SweetCli {

	#[tokio::main]
	pub async fn run(&self) -> Result<()> {
		terminal::clear();
		terminal::print_forky();

		let livereload = LiveReloadLayer::new();
		let reload = livereload.reloader();

		let server2 = self.server.clone();
		let _server_handle = std::thread::spawn(move || -> Result<()> {
			println!("Starting server at {}\n", server2.address.to_string_pretty());
			server2.serve_with_reload(livereload)
		});

		let kill = Arc::new(Mutex::new(()));
		let kill2 = kill.clone();
		let should_kill = move || -> bool { kill2.try_lock().is_ok() };

		loop {
			let change_listener = self.get_change_listener(kill.clone());
			
			if let Err(err) = self.build_wasm(should_kill.clone()) {
				eprintln!("sweet cli: build failed: {:?}", err);
				change_listener.join().unwrap()?;
				continue;
			}

			reload.reload();
			self.handle_run_mode(should_kill.clone()).await?;
			change_listener.join().unwrap()?;
		}
	}

	fn get_change_listener(&self,kill:Arc<Mutex<()>>)->ChangeListener{
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
		})}else{
			// otherwise lock mutex and loop forever
			std::thread::spawn(move ||-> Result<()>{
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