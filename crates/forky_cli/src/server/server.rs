use forky_fs::FsWatcher;
// use forky_fs::FsWatcher;
use futures::Future;
use std::pin::Pin;

// #[derive(Clone)]
pub struct Server {
	pub dir: String,
	pub host: String,
	pub port: u16,
	pub shutdown: Pin<Box<dyn Future<Output = ()>>>,
}

impl Server {
	pub fn with_dir(mut self, dir: &str) -> Self {
		self.dir = dir.to_string();
		self
	}
}

impl Default for Server {
	fn default() -> Self {
		let dir = "html".to_string();
		let host = "0.0.0.0".to_string();
		let port = 3030;

		let dir2 = dir.clone();
		let shutdown = async || {
			FsWatcher::new()
				.with_path(dir2)
				.block_async()
				.await
				.unwrap();
			// std::thread::spawn(|| {
			// 	FsWatcher::new().with_path(dir2).block().unwrap();
			// 	println!("it changed!");
			// let rt = tokio::runtime::Runtime::new().unwrap();
			// rt.spawn(async {
			// 		.unwrap();
			// 	println!("it changed!");
			// });
			// rt.block_on(async {});
			// })
			// .join()
			// .unwrap();
			// tokio::time::sleep(Duration::from_secs(10000)).await;
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
