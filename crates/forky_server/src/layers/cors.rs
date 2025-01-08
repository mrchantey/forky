use axum::body::Body;
use axum::extract::Request;
use axum::http::header;
use axum::http::HeaderValue;
use axum::http::Method;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;

/// TODO handle unwrap
/// Why are we allowing cors?
pub async fn cors(
	req: Request<Body>,
	next: Next,
) -> Result<Response, Response> {
	let origin = req
		.headers()
		.get(header::ORIGIN)
		.map(|i| i.to_str().map(|s| s.to_owned()));

	let is_options = req.method() == Method::OPTIONS;
	let mut res = if is_options {
		let mut res = "".into_response();
		res.headers_mut().insert(
			header::ACCESS_CONTROL_MAX_AGE,
			HeaderValue::from_static("9999999"),
		);
		res
	} else {
		next.run(req).await
	};

	let headers = res.headers_mut();

	if let Some(Ok(origin)) = origin {
		headers.insert(
			header::ACCESS_CONTROL_ALLOW_ORIGIN,
			HeaderValue::from_str(&origin).map_err(|e| {
				(StatusCode::BAD_REQUEST, e.to_string()).into_response()
			})?,
		);
	}

	// headers.insert(
	// 	header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
	// 	HeaderValue::from_static("true"),
	// );

	// headers.insert(
	// 	header::ACCESS_CONTROL_ALLOW_METHODS,
	// 	HeaderValue::from_static("*"),
	// );

	Ok(res)
}
