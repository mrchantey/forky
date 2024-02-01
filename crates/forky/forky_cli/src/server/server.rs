use super::*;
use anyhow::Result;
use axum::http::Method;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use forky_fs::terminal;
use forky_fs::FsWatcher;
use futures::Future;
use hyper::Body;
use hyper::Request;
use std::pin::Pin;
use std::sync::Arc;
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
	pub proxy: bool,
	// pub proxies: Vec<String>,
}

impl Default for Server {
	fn default() -> Self {
		Self {
			dir: "dist".to_string(),
			address: Address::default(),
			clear: true,
			quiet: false,
			any_origin: false,
			proxy: false,
			// proxies:Vec::new(),
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

	pub fn serve_with_default_reload(&self) -> Result<()> {
		let (livereload, _handle) = self.default_reload();
		self.serve_with_reload(livereload)
	}

	pub fn serve_with_reload(&self, livereload: LiveReloadLayer) -> Result<()> {
		self.serve_with_options(Some(livereload))
	}

	#[tokio::main]
	#[rustfmt::skip]
	pub async fn serve_with_options(
		&self,
		livereload: Option<LiveReloadLayer>,
	) -> Result<()> {
		self.print_start();

		let mut router = Router::new()
   	  .route_service("/__ping__", get(ping))
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
		
		if self.proxy{
			let proxy = Arc::new(futures::lock::Mutex::new(Proxy::default()));
			let proxy2 = proxy.clone();
			router = router.nest_service("/_proxy_set_/", get(|req:Request<Body>| async move {
				let mut proxy = proxy.lock().await;
				proxy.handle_set(req)
			}));
			router = router.nest_service("/_proxy_/", get(|req:Request<Body>| async move {
				let proxy = proxy2.lock().await;
				proxy.handle(req).await
			}));
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

	//not yet implemented
	fn _default_shutdown(&self) -> Pin<Box<dyn Future<Output = ()>>> {
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
		let proxy = if self.proxy { "\nproxy: true" } else { "" };
		println!(
			"serving '{}' at {}{any_origin}{proxy}",
			self.dir, self.address
		);
	}
}

async fn ping(req: Request<Body>) -> Response<String> {
	let body = format!("request was {:?}", req);
	Response::new(body)
}
