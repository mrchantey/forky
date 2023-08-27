use anyhow::Result;
use axum::body::Body;
use axum::http::*;
use extend::ext;
use forky_core::OptionTExt;
use hyper::client::Client;

#[ext]
pub impl Result<Response<Body>> {
	fn to_response(self) -> Response<Body> {
		match self {
			Ok(res) => res,
			Err(e) => {
				eprintln!("Response Error: {e}");
				Response::builder()
					.status(StatusCode::BAD_REQUEST)
					.body(Body::from(e.to_string()))
					.unwrap()
			}
		}
	}
}
#[ext]
pub impl Uri {
	fn remove_leading_slash(&self) -> Result<Uri> {
		let uri = self.to_string();
		let uri = without_leading_slash(&uri).to_string();
		Ok(uri.parse()?)
	}
}

fn referrer_is_proxied(req: &Request<Body>) -> bool {
	if let Some(referrer) = req.headers().get("Referer") {
		return referrer.to_str().unwrap_or_default().contains("_proxy_");
	}
	false
}

fn without_leading_slash<'a>(uri: &'a str) -> &'a str {
	if uri.starts_with("/") {
		&uri[1..]
	} else {
		uri
	}
}
fn without_trailing_slash<'a>(uri: &'a str) -> &'a str {
	if uri.ends_with("/") {
		&uri[..(uri.len() - 1)]
	} else {
		uri
	}
}
fn with_leading_slash<'a>(uri: &'a str) -> String {
	if uri.starts_with("/") {
		uri.to_string()
	} else {
		format!("/{uri}")
	}
}

fn without_scheme<'a>(uri: &'a str) -> &'a str {
	if let Some(index) = uri.find("://") {
		&uri[(index + 3)..]
	} else {
		uri
	}
}

pub struct Proxy {
	pub last_uri: Uri,
}

impl Default for Proxy {
	fn default() -> Self {
		Self {
			last_uri: Uri::from_static("http://localhost:3000"),
		}
	}
}

impl Proxy {
	pub fn uri_matches_home(&self, uri: &Uri) -> bool {
		self.last_uri.authority() == uri.authority()
			&& self.last_uri.scheme() == uri.scheme()
	}

	pub fn handle_set(&mut self, req: Request<Body>) -> Response<Body> {
		(|| -> Result<Response<Body>> {
			let uri = req.uri();
			let uri = uri.remove_leading_slash()?;
			self.apply_next_origin(&uri)?;
			Ok(Response::builder()
				.status(StatusCode::OK)
				.body(Body::from("OK"))
				.unwrap())
		})()
		.to_response()
	}

	pub async fn handle(&self, req: Request<Body>) -> Response<Body> {
		(async || -> Result<Response<Body>> {
			let uri = req.uri();
			let uri = uri.remove_leading_slash()?;
			let proxied_referrer = referrer_is_proxied(&req);
			if proxied_referrer {
				let uri_next = self.apply_last_origin(&uri.to_string())?;
				// println!("PROXY - relative:\nfrom: {uri}\nto:   {uri_next}\n");
				handle_proxy_request_from_url(req, &uri_next).await
			} else {
				// println!("Proxy - absolute:\n{uri}\n");
				// self.apply_next_origin(&uri)?;
				handle_proxy_request_from_url(req, &uri).await
			}
		})()
		.await
		.to_response()
	}

	fn apply_last_origin(&self, path: &str) -> Result<Uri> {
		let path = without_scheme(path);
		let path = with_leading_slash(path);
		let path = without_trailing_slash(&path);
		let uri = Uri::builder()
			.authority(self.last_uri.authority().ok()?.clone())
			.scheme(self.last_uri.scheme().ok()?.clone())
			.path_and_query(path)
			.build()?;
		Ok(uri)
	}

	fn apply_next_origin(&mut self, uri: &Uri) -> Result<()> {
		let uri = Uri::builder()
			.authority(
				uri.authority()
					.unwrap_or(self.last_uri.authority().ok()?)
					.clone(),
			)
			.scheme(
				uri.scheme().unwrap_or(self.last_uri.scheme().ok()?).clone(),
			)
			.path_and_query(self.last_uri.path_and_query().ok()?.clone())
			.build()?;
		// println!("URI SET: {uri}");
		self.last_uri = uri;
		Ok(())
	}
}

pub async fn handle_proxy_request_from_url(
	req: Request<Body>,
	uri: &Uri,
) -> Result<Response<Body>> {
	let client = Client::new();
	let mut proxied_req = Request::new(Body::empty());
	*proxied_req.uri_mut() = uri.clone();
	*proxied_req.method_mut() = req.method().clone();
	*proxied_req.headers_mut() = req.headers().clone();
	let proxied_res = client.request(proxied_req).await?;
	Ok(proxied_res)
}
