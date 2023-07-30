// use forky_fs::FsWatcher;
use futures::Future;
use std::pin::Pin;
use std::time::Duration;

// #[derive(Clone)]
pub struct Server {
	pub dir: String,
	pub host: String,
	pub port: u16,
	// pub shutdown: Pin<Box<dyn Fn() -> dyn Future<Output = ()>>>,
	pub shutdown: Pin<Box<dyn Future<Output = ()>>>,
}

impl Server {
	// pub fn dir_watch_shutdown(dir: &str) {
	// 	// FsWatcher::default().
	// }
}

impl Default for Server {
	fn default() -> Self {
		let dir = "html".to_string();
		let host = "0.0.0.0".to_string();
		let port = 3030;

		let shutdown = async || {
			tokio::time::sleep(Duration::from_secs(10000)).await;
		};
		let shutdown = Box::pin(shutdown());
		Self {
			dir,
			shutdown,
			host,
			port,
		}
	}
}
