use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Blob;
use web_sys::BlobPropertyBag;
use web_sys::HtmlInputElement;
use web_sys::Url;

const TYPE_BIN: &str = "application/octet-stream";

pub async fn download_binary(bytes: &[u8]) -> Result<(), JsValue> {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let body = document.body().unwrap();
	let bytes: JsValue = Uint8Array::from(bytes).into();
	let blob = Blob::new_with_u8_array_sequence_and_options(
		&bytes,
		BlobPropertyBag::new().type_(TYPE_BIN),
	)
	.unwrap();
	let url = Url::create_object_url_with_blob(&blob).unwrap();
	let anchor = document
		.create_element("a")
		.unwrap()
		.dyn_into::<web_sys::HtmlAnchorElement>()
		.unwrap();
	anchor.set_attribute("href", &url).unwrap();
	anchor.set_attribute("download", "file.bin").unwrap();
	body.append_child(&anchor).unwrap();
	anchor.click();
	anchor.remove();
	Url::revoke_object_url(&url).unwrap();
	Ok(())
}

pub async fn upload_binary() -> Result<Vec<u8>, JsValue> {
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let body = document.body().unwrap();
	let input = document.create_element("input")?;
	input.set_attribute("type", "file").unwrap();
	input.set_attribute("accept", TYPE_BIN).unwrap();
	input.set_attribute("style", "display:none")?;
	body.append_child(&input)?;
	let promise = input
		.clone()
		.dyn_into::<HtmlInputElement>()?
		.files()
		.unwrap()
		.get(0)
		.unwrap()
		.array_buffer();
	let bytes = JsFuture::from(promise).await?;
	let bytes = Uint8Array::new(&bytes).to_vec();
	body.remove_child(&input)?;
	input.remove();
	Ok(bytes)
}
