use extend::ext;
use wasm_bindgen::JsValue;

#[ext]
pub impl<T> Result<T, JsValue> {
	fn anyhow(self) -> anyhow::Result<T> {
		match self {
			Ok(v) => Ok(v),
			Err(e) => Err(anyhow::anyhow!("{:?}", e)),
		}
	}
}
