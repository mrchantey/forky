use crate::terminal;
use anyhow::Result;
use forky_core::*;
use futures::SinkExt;
use futures::StreamExt;
use notify::Config;
use notify::Event;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use notify::*;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::DebouncedEvent;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct FsWatcher {
	pub path: String,
	pub interval: Duration,
	// pub debounce: Duration,
	pub run_on_start: bool,
	pub quiet: bool,
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
			quiet: false,
			once_per_tick: true,
			path: String::from("./"),
			interval: Duration::from_secs(1),
			watches: Vec::new(),
			ignores: Vec::new(),
		}
	}
}

impl FsWatcher {
	pub fn new() -> Self { Self::default() }

	pub fn with_path(mut self, path: String) -> Self {
		self.path = path;
		self
	}
	pub fn with_quiet(mut self, quiet: bool) -> Self {
		self.quiet = quiet;
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
		if !self.quiet {
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

	fn handle_rx(
		&self,
		res: Result<Event, Error>,
		(start, last_run): &mut (Instant, Instant),
		on_change: impl Fn(&str) -> Result<()>,
	) -> Result<()> {
		let now = Instant::now();
		let last_elapsed = now.duration_since(*last_run);
		if last_elapsed < self.interval {
			return Ok(());
		}

		match res {
			Ok(e) => {
				let _mutex = self.lock();
				let last_run2 = last_run;
				let start_elapsed = now.duration_since(*start).as_secs_f32();
				e.paths
					.iter()
					.filter(|path| self.passes(&path))
					.take(if self.once_per_tick { 1 } else { usize::MAX })
					.map(|path| {
						self.prep_terminal();
						if !self.quiet {
							println!(
								"{:.2} - file changed: {}\n",
								start_elapsed,
								path.file_name().unwrap().to_str().unwrap()
							)
						}
						on_change(path.to_str().ok()?)?;
						// now after on_change in case its long
						*last_run2 = Instant::now();
						Ok(())
					})
					.collect::<Result<()>>()?;
			}
			Err(e) => eprintln!("watch error: {:?}", e),
		}
		Ok(())
	}

	pub fn block(&self) -> Result<()> {
		let (_watcher, rx) = self.watcher()?;
		let mut timers = timers();

		for res in rx {
			self.handle_rx(res, &mut timers, |_| Ok(()))?;
			return Ok(());
		}
		Ok(())
	}
	pub async fn block_async(&self) -> Result<()> {
		let (_watcher, mut rx) = self.watcher_async()?;
		let mut timers = timers();

		while let Some(res) = rx.next().await {
			self.handle_rx(res, &mut timers, |_| Ok(()))?;
			return Ok(());
		}
		Ok(())
	}

	pub fn watch(&self, on_change: impl Fn(&str) -> Result<()>) -> Result<()> {
		self.try_run_on_start(&on_change)?;
		let (_watcher, rx) = self.watcher()?;
		let mut timers = timers();

		for res in rx {
			self.handle_rx(res, &mut timers, &on_change)?;
		}
		Ok(())
	}
	pub async fn watch_async(
		&self,
		on_change: impl Fn(&str) -> Result<()>,
	) -> Result<()> {
		self.try_run_on_start(&on_change)?;
		let (_watcher, mut rx) = self.watcher_async()?;
		let mut timers = timers();

		while let Some(res) = rx.next().await {
			self.handle_rx(res, &mut timers, &on_change)?;
		}
		Ok(())
	}

	fn try_run_on_start(
		&self,
		on_change: impl Fn(&str) -> Result<()>,
	) -> Result<()> {
		if self.run_on_start {
			let _mutex = self.lock();
			self.prep_terminal();
			on_change("")?;
		}
		Ok(())
	}

	fn watcher(
		&self,
	) -> Result<(
		RecommendedWatcher,
		std::sync::mpsc::Receiver<notify::Result<Event>>,
	)> {
		let path = Path::new(&self.path);

		let (tx, rx) = std::sync::mpsc::channel();
		let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
		watcher.watch(path, RecursiveMode::Recursive)?;

		Ok((watcher, rx))
	}

	fn watcher_async(
		&self,
	) -> Result<(
		RecommendedWatcher,
		futures::channel::mpsc::Receiver<notify::Result<Event>>,
	)> {
		let (mut tx, rx) = futures::channel::mpsc::channel(1);
		let mut watcher = RecommendedWatcher::new(
			move |res| {
				futures::executor::block_on(async {
					if let Err(err) = tx.send(res).await {
						eprintln!("{:?}", err);
					}
				})
			},
			Config::default(),
		)?;
		let path = Path::new(&self.path);
		watcher.watch(path, RecursiveMode::Recursive)?;

		Ok((watcher, rx))
	}
}


fn timers() -> (Instant, Instant) {
	let start = Instant::now();
	let last_run = start;
	(start, last_run)
}
