use notify::Config;
use notify::PollWatcher;
use notify::*;
use std::path::Path;
use std::time::Duration;

//no longer used, cargo watch is plenty
pub fn log_changes(path: &str, on_change: fn(e: notify::Event)) {
	let (tx, rx) = std::sync::mpsc::channel();
	let config = Config::default()
		.with_poll_interval(Duration::from_millis(500))
		.with_compare_contents(true);
	let mut watcher = PollWatcher::new(tx, config).unwrap();
	let path = Path::new(path);
	watcher.watch(path, RecursiveMode::Recursive).unwrap();

	loop {
		match rx.recv() {
			Ok(e) => on_change(e.unwrap()),
			Err(e) => println!("watch error: {:?}", e),
		}
	}
}
