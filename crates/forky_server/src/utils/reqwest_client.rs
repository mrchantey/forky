use bytes::Bytes;
use once_cell::sync::Lazy;
pub use reqwest::Client;



pub static REQWEST_CLIENT: Lazy<Client> = Lazy::new(|| {
	Client::builder()
		// .user_agent(USER_AGENT)
		.build()
		.expect("Failed to build reqwest client")
});


pub struct ReqwestExt;

impl ReqwestExt {
	pub fn uri_to_url(uri: http::Uri) -> reqwest::Url {
		reqwest::Url::parse(&uri.to_string())
			.expect("any valid uri should be a valid url")
	}


	pub async fn from_http_request_collect_body<T: http_body::Body>(
		req: http::Request<T>,
	) -> Result<reqwest::Request, T::Error> {
		let (parts, body) = req.into_parts();
		let bytes = http_body_util::BodyExt::collect(body)
			.await
			.map(|buf| buf.to_bytes())?;

		let req = http::Request::from_parts(parts, bytes);

		let req = Self::from_http_request(req);
		Ok(req)
	}

	fn from_http_request<T: Into<bytes::Bytes>>(
		req: http::Request<T>,
	) -> reqwest::Request {
		let (
			http::request::Parts {
				method,
				uri,
				version,
				headers,
				// extensions,
				..
			},
			body,
		) = req.into_parts();
		let url = Self::uri_to_url(uri);
		let mut reqwest_req = reqwest::Request::new(method, url);
		*reqwest_req.headers_mut() = headers;
		*reqwest_req.version_mut() = version;
		let bytes: Bytes = body.into();
		*reqwest_req.body_mut() = Some(bytes.into());
		reqwest_req
	}

	pub async fn into_http_response(
		res: reqwest::Response,
	) -> Result<http::Response<Bytes>, reqwest::Error> {
		let response: http::Response<reqwest::Body> = res.into();
		let (parts, streaming_body) = response.into_parts();
		let body = http_body_util::BodyExt::collect(streaming_body)
			.await
			.map(|buf| buf.to_bytes())?;

		let response = http::Response::from_parts(parts, body);
		Ok(response)
	}
}
