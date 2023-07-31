use anyhow::Result;
use axum::Router;
use forky_fs::terminal;
use forky_fs::FsWatcher;
use futures::Future;
use std::pin::Pin;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

// #[derive(Clone)]
pub struct Server {
	pub dir: String,
	pub host: String,
	pub port: u16,
}

impl Server {
	pub fn with_dir(mut self, dir: &str) -> Self {
		self.dir = dir.to_string();
		self
	}
	pub fn serve_forever(&self) -> Result<()> {
		loop {
			self.serve()?;
		}
	}

	#[tokio::main]
	pub async fn serve(&self) -> Result<()> {
		let app = Router::new()
			.nest_service("/", ServeDir::new(self.dir.as_str()))
			.layer(LiveReloadLayer::new());

		self.print_start();
		let addr = format!("{}:{}", self.host, self.port);
		axum::Server::bind(&addr.parse()?)
			.serve(app.into_make_service())
			.with_graceful_shutdown(self.get_shutdown())
			.await?;
		Ok(())
	}

	fn get_shutdown(&self) -> Pin<Box<dyn Future<Output = ()>>> {
		let dir = self.dir.clone();
		let shutdown = async || {
			FsWatcher::new().with_path(dir).block_async().await.unwrap();
		};
		Box::pin(shutdown())
	}

	fn print_start(&self) {
		terminal::clear();
		terminal::print_forky();
		let host = if self.host == "0.0.0.0" {
			"127.0.0.1"
		} else {
			self.host.as_str()
		};
		let addr = format!("http://{host}:{}", self.port);
		println!("serving '{}' at {addr}", self.dir);
	}
}

impl Default for Server {
	fn default() -> Self {
		let dir = "html".to_string();
		let host = "0.0.0.0".to_string();
		let port = 3030;
		Self { dir, host, port }
	}
}
