use anyhow::Result;
use notify::Config;
use notify::PollWatcher;
use notify::*;
use std::path::Path;
use std::time::Duration;


// pub type WatchFunc = Fn(notify::Event);

#[derive(Debug)]
pub struct WatchConfig {
	pub path: String,
	pub interval: Duration,
	/// glob for watch pattern
	pub watch: Option<glob::Pattern>,
	/// glob for ignore pattern
	pub ignore: Option<glob::Pattern>,
}

impl Default for WatchConfig {
	fn default() -> Self {
		Self {
			path: String::from("./"),
			interval: Duration::from_millis(500),
			watch: None,
			ignore: None,
		}
	}
}

impl WatchConfig {
	pub fn passes(&self, path: &Path) -> bool {
		self.passes_watch(path) && self.passes_ignore(path)
	}

	pub fn passes_watch(&self, path: &Path) -> bool {
		match &self.watch {
			Some(watch) => watch.matches_path(path),
			None => true,
		}
	}

	pub fn passes_ignore(&self, path: &Path) -> bool {
		match &self.ignore {
			Some(ignore) => !ignore.matches_path(path),
			None => true,
		}
	}
}

pub fn watch_path_log(config: WatchConfig) -> Result<()> {
	watch_path(config, |e| println!("changed: {:?}", e))
}

pub fn watch_path(
	config: WatchConfig,
	on_change: impl Fn(notify::Event),
) -> Result<()> {
	watch_path_only(&config, |e| {
		if let Some(watch) = &config.watch {
			if e.paths.iter().any(|path| watch.matches_path(path)) {
				on_change(e)
			}
		}
	})
}

pub fn watch_path_only(
	config: &WatchConfig,
	on_change: impl Fn(notify::Event),
) -> Result<()> {
	let (tx, rx) = std::sync::mpsc::channel();
	let w_config = Config::default()
		.with_poll_interval(config.interval)
		.with_compare_contents(true);
	let mut watcher = RecommendedWatcher::new(tx, w_config)?;
	// let mut watcher = PollWatcher::new(tx, config).unwrap();
	let path = Path::new(config.path.as_str());
	watcher.watch(path, RecursiveMode::Recursive)?;

	for res in rx {
		match res {
			Ok(e) => on_change(e),
			Err(e) => println!("watch error: {:?}", e),
		}
	}
	Ok(())
}
