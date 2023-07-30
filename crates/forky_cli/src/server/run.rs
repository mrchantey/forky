use super::Server;
use anyhow::Result;
use axum::Router;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

impl Server {
	pub fn serve_forever(make_server: impl Fn() -> Server) -> Result<()> {
		loop {
			make_server().serve()?;
		}
	}
	#[tokio::main]
	pub async fn serve(self) -> Result<()> {
		let app = Router::new()
			.nest_service("/", ServeDir::new(self.dir.as_str()))
			.layer(LiveReloadLayer::new());
		let addr = format!("{}:{}", self.host, self.port);
		println!("serving\n{addr}");

		axum::Server::bind(&addr.parse()?)
			.serve(app.into_make_service())
			.with_graceful_shutdown(self.shutdown)
			.await?;
		Ok(())
	}
}
