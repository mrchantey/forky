use crate::terminal;
use anyhow::Result;
use forky_core::*;
use notify::*;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::Watcher;
use notify_debouncer_full::DebouncedEvent;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::Duration;

#[derive(Debug)]
pub struct FsWatcher {
	pub path: String,
	pub interval: Duration,
	pub run_on_start: bool,
	pub clear_on_change: bool,
	pub log_changed_file: bool,
	/// glob for watch patterns
	pub watch: Vec<glob::Pattern>,
	pub mutex: Option<ArcMut<()>>,
	/// glob for ignore patterns
	pub ignore: Vec<glob::Pattern>,
}

impl Default for FsWatcher {
	fn default() -> Self {
		Self {
			mutex: None,
			run_on_start: true,
			clear_on_change: true,
			log_changed_file: true,
			path: String::from("./"),
			interval: Duration::from_millis(10),
			watch: Vec::new(),
			ignore: Vec::new(),
		}
	}
}

impl FsWatcher {
	pub fn new() -> Self { Self::default() }

	pub fn with_mutex(mut self, mutex: ArcMut<()>) -> Self {
		self.mutex = Some(mutex);
		self
	}
	pub fn with_watch(mut self, watch: &str) -> Self {
		self.watch.push(glob::Pattern::new(watch).unwrap());
		self
	}
	pub fn with_ignore(mut self, watch: &str) -> Self {
		self.ignore.push(glob::Pattern::new(watch).unwrap());
		self
	}
	pub fn passes(&self, path: &Path) -> bool {
		self.passes_watch(path) && self.passes_ignore(path)
	}

	pub fn passes_watch(&self, path: &Path) -> bool {
		self.watch.iter().any(|watch| watch.matches_path(path))
	}

	pub fn passes_ignore(&self, path: &Path) -> bool {
		!self.ignore.iter().any(|watch| watch.matches_path(path))
	}
	pub fn watch_log(&self) -> Result<()> {
		self.watch(|e| {
			println!("changed: {:?}", e);
			Ok(())
		})
	}
	fn prep_terminal(&self) {
		if self.clear_on_change {
			terminal::clear();
		}
		terminal::print_forky();
	}
	fn lock(&self) -> Option<MutexGuard<()>> {
		self.mutex.as_ref().map(|m| m.lock().unwrap())
	}

	pub fn watch(&self, on_change: impl Fn(&str) -> Result<()>) -> Result<()> {
		if self.run_on_start {
			self.prep_terminal();
			on_change("")?;
		}

		let (tx, rx) = std::sync::mpsc::channel();
		let path = Path::new(&self.path);
		let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;
		let watcher =
			debouncer.watcher().watch(path, RecursiveMode::Recursive)?;
		debouncer.cache().add_root(path, RecursiveMode::Recursive);

		for res in rx {
			match res {
				Ok(e) => {
					let _ = self.lock();
					e.iter()
						.flat_map(|e| {
							e.paths.iter().map(move |p| (p.clone(), e))
						})
						.filter(|(path, e)| self.passes(&path))
						.map(|(path, e)| {
							self.prep_terminal();
							if self.log_changed_file {
								println!("{:?}: {:?}", e.kind, path)
							}
							on_change(path.to_str().ok()?)
						})
						.collect::<Result<()>>()?;
				}
				Err(e) => println!("watch error: {:?}", e),
			}
		}
		Ok(())
	}
	// pub fn watch_poll(&self, on_change: impl Fn(&PathBuf)) -> Result<()> {
	// 	let (tx, rx) = std::sync::mpsc::channel();
	// 	let w_config = Config::default().with_poll_interval(self.interval);
	// 	let mut watcher = RecommendedWatcher::new(tx, w_config)?;
	// 	// let mut watcher = PollWatcher::new(tx, config).unwrap();
	// 	let path = Path::new(config.path.as_str());
	// 	watcher.watch(path, RecursiveMode::Recursive)?;

	// 	for res in rx {
	// 		match res {
	// 			Ok(e) => {
	// 				e.paths
	// 					.iter()
	// 					.filter(|path| self.passes(path))
	// 					.for_each(|e| on_change(e));
	// 			}
	// 			Err(e) => println!("watch error: {:?}", e),
	// 		}
	// 	}
	// 	Ok(())
	// }
}
