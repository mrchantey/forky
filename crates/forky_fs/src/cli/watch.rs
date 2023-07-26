use anyhow::Result;
use notify::*;
use notify_debouncer_full::notify::Watcher;
// use notify::*;
use notify_debouncer_full::{
	new_debouncer,
	DebouncedEvent,
};
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;


// pub type WatchFunc = Fn(notify::Event);

#[derive(Debug)]
pub struct WatchConfig {
	pub path: String,
	pub interval: Duration,
	/// glob for watch pattern
	pub watch: Vec<glob::Pattern>,
	/// glob for ignore pattern
	pub ignore: Vec<glob::Pattern>,
}

impl Default for WatchConfig {
	fn default() -> Self {
		Self {
			path: String::from("./"),
			interval: Duration::from_millis(500),
			watch: Vec::new(),
			ignore: Vec::new(),
		}
	}
}

impl WatchConfig {
	pub fn passes(&self, path: &Path) -> bool {
		self.passes_watch(path) && self.passes_ignore(path)
	}

	pub fn passes_watch(&self, path: &Path) -> bool {
		self.watch.iter().any(|watch| watch.matches_path(path))
	}

	pub fn passes_ignore(&self, path: &Path) -> bool {
		!self.ignore.iter().any(|watch| watch.matches_path(path))
	}
}

pub fn watch_path_log(config: &WatchConfig) -> Result<()> {
	watch_path(config, |e| println!("changed: {:?}", e))
}

pub fn watch_path(
	config: &WatchConfig,
	on_change: impl Fn(&str),
) -> Result<()> {
	let (tx, rx) = std::sync::mpsc::channel();
	let path = Path::new(&config.path);
	let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;
	let watcher = debouncer.watcher().watch(path, RecursiveMode::Recursive)?;
	debouncer.cache().add_root(path, RecursiveMode::Recursive);

	for res in rx {
		match res {
			Ok(e) => {
				e.iter()
					.flat_map(|p| p.paths.clone())
					.filter(|path| config.passes(&path))
					.for_each(|e| on_change(e.to_str().unwrap()));
			}
			Err(e) => println!("watch error: {:?}", e),
		}
	}
	Ok(())
}
pub fn watch_path_poll(
	config: &WatchConfig,
	on_change: impl Fn(&PathBuf),
) -> Result<()> {
	let (tx, rx) = std::sync::mpsc::channel();
	let w_config = Config::default().with_poll_interval(config.interval);
	let mut watcher = RecommendedWatcher::new(tx, w_config)?;
	// let mut watcher = PollWatcher::new(tx, config).unwrap();
	let path = Path::new(config.path.as_str());
	watcher.watch(path, RecursiveMode::Recursive)?;

	for res in rx {
		match res {
			Ok(e) => {
				e.paths
					.iter()
					.filter(|path| config.passes(path))
					.for_each(|e| on_change(e));
			}
			Err(e) => println!("watch error: {:?}", e),
		}
	}
	Ok(())
}
