use super::*;
use anyhow::Result;
use axum::http::Method;
use axum::Router;
use forky_fs::terminal;
use forky_fs::FsWatcher;
use futures::Future;
use std::pin::Pin;
use std::thread::JoinHandle;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[derive(Debug, Clone)]
pub struct Server {
	pub dir: String,
	pub address: Address,
	pub clear: bool,
	pub quiet: bool,
	pub any_origin: bool,
}

impl Default for Server {
	fn default() -> Self {
		Self {
			dir: "dist".to_string(),
			address: Address::default(),
			clear: true,
			quiet: false,
			any_origin: false,
		}
	}
}

impl Server {
	pub fn with_dir(mut self, dir: &str) -> Self {
		self.dir = dir.to_string();
		self
	}
	pub fn quietly(mut self) -> Self {
		self.quiet = true;
		self
	}

	pub fn serve_with_shutdown(
		&self,
		shutdown: impl Future<Output = ()>,
	) -> Result<()> {
		self.serve_with_callbacks(Some(shutdown), None)
	}

	pub fn serve_with_default_reload(&self) -> Result<()> {
		let (livereload, _handle) = self.default_reload();
		self.serve_with_reload(livereload)
	}

	pub fn serve_with_reload(&self, livereload: LiveReloadLayer) -> Result<()> {
		#[allow(unused_assignments)]
		let mut shutdown = Some(self.default_shutdown());
		shutdown = None;
		self.serve_with_callbacks(shutdown, Some(livereload))
	}

	#[tokio::main]
	#[rustfmt::skip]
	pub async fn serve_with_callbacks(
		&self,
		// unused for now
		_shutdown: Option<impl Future<Output = ()>>,
		livereload: Option<LiveReloadLayer>,
	) -> Result<()> {
		self.print_start();

		let mut router = Router::new()
			.nest_service("/", ServeDir::new(self.dir.as_str()));

		if let Some(livereload) = livereload {
			router = router.layer(livereload);
		}
		if self.any_origin{
			let cors = CorsLayer::new()
			.allow_methods([Method::GET, Method::POST])
			.allow_origin(tower_http::cors::Any);
			router = router.layer(cors);
		}

		if self.address.secure{
			self.serve_secure(router).await?;
		}else{
		self.serve_insecure(router).await?;
		}
		Ok(())
	}

	fn default_reload(&self) -> (LiveReloadLayer, JoinHandle<Result<()>>) {
		let livereload = LiveReloadLayer::new();
		let reload = livereload.reloader();
		let this = self.clone();
		let reload_handle = std::thread::spawn(move || -> Result<()> {
			let mut this2 = this.clone();
			this2.clear = false;
			FsWatcher::new()
				.with_path(this.dir)
				.with_quiet(this.quiet)
				.watch(move |_| {
					reload.reload();
					this2.print_start();
					Ok(())
				})
		});
		(livereload, reload_handle)
	}


	fn default_shutdown(&self) -> Pin<Box<dyn Future<Output = ()>>> {
		let dir = self.dir.clone();
		let quiet = self.quiet;
		Box::pin(async move {
			FsWatcher::new()
				.with_path(dir)
				.with_quiet(quiet)
				.block_async()
				.await
				.unwrap();
		})
	}

	fn print_start(&self) {
		if self.quiet {
			return;
		}
		if self.clear {
			terminal::clear();
			terminal::print_forky();
		}
		let any_origin = if self.any_origin {
			"\nany-origin: true"
		} else {
			""
		};
		println!("serving '{}' at {}{any_origin}", self.dir, self.address);
	}
}
