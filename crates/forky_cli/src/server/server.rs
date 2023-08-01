use anyhow::Result;
use axum::Router;
use forky_fs::terminal;
use forky_fs::FsWatcher;
use futures::Future;
use std::pin::Pin;
use std::thread::JoinHandle;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[derive(Clone)]
pub struct Server {
	pub dir: String,
	pub host: String,
	pub port: u16,
	pub clear: bool,
	pub quiet: bool,
}

impl Default for Server {
	fn default() -> Self {
		Self {
			dir: "html".to_string(),
			host: "0.0.0.0".to_string(),
			port: 3030,
			clear: true,
			quiet: false,
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

	pub fn serve(&self) -> Result<()> {
		loop {
			self.serve_with_shutdown(self.default_shutdown())?;
		}
	}

	#[tokio::main]
	pub async fn serve_with_shutdown(
		&self,
		shutdown: impl Future<Output = ()>,
	) -> Result<()> {
		let app =
			Router::new().nest_service("/", ServeDir::new(self.dir.as_str()));

		self.print_start();
		let addr = format!("{}:{}", self.host, self.port);
		axum::Server::bind(&addr.parse()?)
			.serve(app.into_make_service())
			.with_graceful_shutdown(shutdown)
			.await?;
		Ok(())
	}

	#[tokio::main]
	pub async fn serve_with_reload(
		&self,
		livereload: LiveReloadLayer,
	) -> Result<()> {
		let app = Router::new()
			.nest_service("/", ServeDir::new(self.dir.as_str()))
			.layer(livereload);

		self.print_start();
		let addr = format!("{}:{}", self.host, self.port);
		axum::Server::bind(&addr.parse()?)
			.serve(app.into_make_service())
			.await?;
		Ok(())
	}

	pub fn serve_with_default_reload(&self) -> Result<()> {
		let (livereload, _handle) = self.default_relaod();
		self.serve_with_reload(livereload)
	}

	fn default_relaod(&self) -> (LiveReloadLayer, JoinHandle<Result<()>>) {
		let livereload = LiveReloadLayer::new();
		let reload = livereload.reloader();
		let this = self.clone();
		// let dir = self.dir.clone();
		// let quiet = self.quiet;
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
		let host = if self.host == "0.0.0.0" {
			"127.0.0.1"
		} else {
			self.host.as_str()
		};
		let addr = format!("http://{host}:{}", self.port);
		println!("serving '{}' at {addr}", self.dir);
	}
}
