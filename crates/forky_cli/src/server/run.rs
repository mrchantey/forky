use super::Server;
use anyhow::Result;
use axum::Router;
use forky_fs::terminal;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

impl Server {
	pub fn serve_forever(make_server: impl Fn() -> Server) -> Result<()> {
		loop {
			make_server().serve()?;
		}
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

	#[tokio::main]
	pub async fn serve(self) -> Result<()> {
		let app = Router::new()
			.nest_service("/", ServeDir::new(self.dir.as_str()))
			.layer(LiveReloadLayer::new());

		self.print_start();
		let addr = format!("{}:{}", self.host, self.port);
		axum::Server::bind(&addr.parse()?)
			.serve(app.into_make_service())
			.with_graceful_shutdown(self.shutdown)
			.await?;
		Ok(())
	}
}
