use super::*;
use anyhow::Result;
use axum::extract::Host;
use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::Redirect;
use axum::BoxError;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

impl Server {
	pub async fn serve_insecure(&self, router: Router) -> Result<()> {
		axum::Server::bind(&self.address.to_socket_addr())
			.serve(router.into_make_service())
			.await?;
		Ok(())
	}

	pub async fn serve_secure(&self, router: Router) -> Result<()> {
		let cert = include_bytes!("self_signed_certs___/cert.pem");
		let key = include_bytes!("self_signed_certs___/key.pem");

		let config =
			RustlsConfig::from_pem(cert.to_vec(), key.to_vec()).await?;

		axum_server::bind_rustls(self.address.to_socket_addr_tls(), config)
			.serve(router.into_make_service())
			.await?;
		Ok(())
	}
}

pub async fn redirect_http_to_https(ports: Address) -> Result<()> {
	let redirect = move |Host(host): Host, uri: Uri| async move {
		match make_https(host, uri, ports) {
			Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
			Err(error) => {
				eprintln!("failed to convert URI to HTTPS: {}", error);
				Err(StatusCode::BAD_REQUEST)
			}
		}
	};

	let addr = SocketAddr::from(([127, 0, 0, 1], ports.port));
	axum::Server::bind(&addr)
		.serve(redirect.into_make_service())
		.await?;
	Ok(())
}

fn make_https(host: String, uri: Uri, ports: Address) -> Result<Uri, BoxError> {
	let mut parts = uri.into_parts();

	parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

	if parts.path_and_query.is_none() {
		parts.path_and_query = Some("/".parse().unwrap());
	}

	let https_host =
		host.replace(&ports.port.to_string(), &ports.port_tls.to_string());
	parts.authority = Some(https_host.parse()?);

	Ok(Uri::from_parts(parts)?)
}
