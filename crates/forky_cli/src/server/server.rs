use super::*;
use anyhow::Result;
use axum::extract::Request;
use axum::http::Method;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use forky_fs::prelude::*;
use futures::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::thread::JoinHandle;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use tower_livereload::LiveReloadLayer;

/// Serve static files
#[derive(Debug, Clone, Parser)]
pub struct Server {
	/// Directory to serve
	#[arg(default_value = "./")]
	pub dir: String,
	/// Specify port
	#[arg(long, default_value = "3000")]
	port: String,
	/// Specify host
	#[arg(long, default_value = "0.0.0.0")]
	host: String,
	/// Run with https
	#[arg(long)]
	secure: bool,
	// pub address: Address,
	#[arg(long, default_value = "true")]
	pub clear: bool,
	pub quiet: bool,
	/// If a url is not found, do not fallback to index.html
	#[arg(long)]
	pub no_fallback: bool,
	/// Add 'access-control-allow-origin: *' header
	#[arg(long)]
	any_origin: bool,
	/// Adds a proxy served from /_proxy/*
	#[arg(long)]
	proxy: bool,
	// pub proxies: Vec<String>,
}

impl Server {
	pub fn run(self) -> Result<()> { self.serve_with_default_reload() }

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
	pub async fn serve_with_options(
		&self,
		livereload: Option<LiveReloadLayer>,
	) -> Result<()> {
		self.print_start();

		let mut router = Router::new().route_service("/__ping__", get(ping));

		if self.no_fallback {
			router = router.nest_service("/", ServeDir::new(self.dir.as_str()));
		} else {
			router = router.nest_service(
				"/",
				ServeDir::new(self.dir.as_str())
					.fallback(ServeFile::new("index.html")),
			);
		}
		if let Some(livereload) = livereload {
			router = router.layer(livereload);
		}
		if self.any_origin {
			let cors = CorsLayer::new()
				.allow_methods([Method::GET, Method::POST])
				.allow_origin(tower_http::cors::Any);
			router = router.layer(cors);
		}

		if self.proxy {
			let proxy = Arc::new(futures::lock::Mutex::new(Proxy::default()));
			let proxy2 = proxy.clone();
			router = router.nest_service(
				"/_proxy_set_/",
				get(|req: Request| async move {
					let mut proxy = proxy.lock().await;
					proxy.handle_set(req)
				}),
			);
			router = router.nest_service(
				"/_proxy_/",
				get(|req: Request| async move {
					let proxy = proxy2.lock().await;
					proxy.handle(req).await
				}),
			);
		}

		if self.secure {
			self.serve_secure(router).await?;
		} else {
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
			self.dir,
			self.address().unwrap(),
		);
	}
	pub fn address(&self) -> Result<Address> {
		Ok(Address {
			host: Address::host_from_str(&self.host)?,
			port: self.port.parse::<u16>()?,
			secure: self.secure,
			..Default::default()
		})
	}
}

async fn ping(req: Request) -> Response<String> {
	let body = format!("request was {:?}", req);
	Response::new(body)
}
