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
	// pub debounce: Duration,
	pub run_on_start: bool,
	pub clear_on_change: bool,
	pub log_changed_file: bool,
	pub once_per_tick: bool,
	/// glob for watch patterns
	pub watches: Vec<glob::Pattern>,
	pub mutex: Option<ArcMut<()>>,
	/// glob for ignore patterns
	pub ignores: Vec<glob::Pattern>,
}

impl Default for FsWatcher {
	fn default() -> Self {
		Self {
			mutex: None,
			run_on_start: true,
			clear_on_change: true,
			log_changed_file: true,
			once_per_tick: true,
			path: String::from("./"),
			interval: Duration::from_millis(10),
			watches: Vec::new(),
			ignores: Vec::new(),
		}
	}
}

impl FsWatcher {
	pub fn new() -> Self { Self::default() }

	pub fn with_dont_clear(mut self) -> Self {
		self.clear_on_change = false;
		self
	}

	pub fn with_mutex(mut self, mutex: ArcMut<()>) -> Self {
		self.mutex = Some(mutex);
		self
	}
	pub fn with_watches(mut self, watch: Vec<&str>) -> Self {
		self.watches = watch
			.iter()
			.map(|w| glob::Pattern::new(w).unwrap())
			.collect();
		self
	}
	pub fn with_ignores(mut self, ignore: Vec<&str>) -> Self {
		self.ignores = ignore
			.iter()
			.map(|w| glob::Pattern::new(w).unwrap())
			.collect();
		self
	}

	pub fn with_watch(mut self, watch: &str) -> Self {
		self.watches.push(glob::Pattern::new(watch).unwrap());
		self
	}
	pub fn with_ignore(mut self, watch: &str) -> Self {
		self.ignores.push(glob::Pattern::new(watch).unwrap());
		self
	}
	pub fn passes(&self, path: &Path) -> bool {
		self.passes_watch(path) && self.passes_ignore(path)
	}

	pub fn passes_watch(&self, path: &Path) -> bool {
		self.watches.iter().any(|watch| watch.matches_path(path))
			|| self.watches.is_empty()
	}

	pub fn passes_ignore(&self, path: &Path) -> bool {
		!self.ignores.iter().any(|watch| watch.matches_path(path))
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
			terminal::print_forky();
			println!(
				"watching...\nwatching: {:?}\nignoring: {:?}\n",
				self.watches.iter().map(|w| w.as_str()).collect::<Vec<_>>(),
				self.ignores.iter().map(|w| w.as_str()).collect::<Vec<_>>()
			);
		}
	}
	fn lock(&self) -> Option<MutexGuard<()>> {
		self.mutex.as_ref().map(|m| m.lock().unwrap())
	}

	pub fn watch(&self, on_change: impl Fn(&str) -> Result<()>) -> Result<()> {
		if self.run_on_start {
			let _mutex = self.lock();
			self.prep_terminal();
			on_change("")?;
		}

		let (tx, rx) = std::sync::mpsc::channel();
		let path = Path::new(&self.path);
		let mut debouncer = new_debouncer(self.interval, None, tx)?;
		let watcher =
			debouncer.watcher().watch(path, RecursiveMode::Recursive)?;
		debouncer.cache().add_root(path, RecursiveMode::Recursive);

		let start = std::time::Instant::now();
		let mut last_run = start;

		for res in rx {
			match res {
				Ok(e) => {
					let _mutex = self.lock();
					let last_run2 = last_run;
					e.iter()
						.filter(|e| {
							e.time.duration_since(last_run2) >= self.interval
						})
						.flat_map(|e| {
							e.paths.iter().map(move |p| (p.clone(), e))
						})
						.filter(|(path, e)| self.passes(&path))
						.take(if self.once_per_tick { 1 } else { usize::MAX })
						.map(|(path, e)| {
							self.prep_terminal();
							if self.log_changed_file {
								println!(
									"{:.2}: {:?}: {:?}\n",
									e.time.duration_since(start).as_secs_f32(),
									e.kind,
									path.file_name().unwrap()
								)
							}
							on_change(path.to_str().ok()?)?;
							last_run = std::time::Instant::now();
							Ok(())
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
