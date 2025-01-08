use axum::extract::Request;
use axum::http::header;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;

/// Middleware to add no-cache headers to a response
pub async fn no_cache(request: Request, next: Next) -> Response {
	let response = next.run(request).await;
	append_no_cache_headers(response)
}

/// Append no-cache headers to a response
pub fn append_no_cache_headers(val: impl IntoResponse) -> Response {
	let mut response = val.into_response();
	let headers = response.headers_mut();
	headers.insert(
		header::CACHE_CONTROL,
		HeaderValue::from_static("no-cache, no-store, must-revalidate"),
	);
	headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
	headers.insert(header::EXPIRES, HeaderValue::from_static("0"));
	// do something with `response`...

	response
}

/// "latest" is a special version that should not be cached
pub fn append_no_cache_headers_if_latest(
	val: impl IntoResponse,
	version: &str,
) -> Response {
	if version == "latest" {
		append_no_cache_headers(val)
	} else {
		val.into_response()
	}
}
/// "latest" is a special version that should not be cached
/// Also branches are not cached because they resolve to the latest commit
pub fn append_no_cache_headers_if_latest_or_branch(
	val: impl IntoResponse,
	version: &str,
) -> Response {
	if version == "latest" {
		append_no_cache_headers(val)
	} else if version.len() != 40 {
		append_no_cache_headers(val)
	} else {
		val.into_response()
	}
}
