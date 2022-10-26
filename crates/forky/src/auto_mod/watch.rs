use notify::poll::PollWatcherConfig;
use notify::*;
use std::{path::Path, time::Duration};

pub fn log_changes(path: &str, on_change: fn()) {
	let (tx, rx) = std::sync::mpsc::channel();
	let config = PollWatcherConfig {
		compare_contents: true,
		poll_interval: Duration::from_millis(500),
	};
	let mut watcher = PollWatcher::with_config(tx, config).unwrap();
	let path = Path::new(path);
	watcher.watch(path, RecursiveMode::Recursive).unwrap();

	loop {
		match rx.recv() {
			Ok(event) => {
				// println!("{:?}", event.unwrap().kind)
				on_change();
			}
			Err(e) => println!("watch error: {:?}", e),
		}
	}
}
