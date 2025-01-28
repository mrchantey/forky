use crate::*;
use anyhow::Result;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;


pub async fn fetch(url: &str) -> Result<Response> {
	let window = web_sys::window().unwrap();
	let prom = window.fetch_with_str(url);

	let res = JsFuture::from(prom).await.anyhow()?;
	let res = res.dyn_into().anyhow()?;
	Ok(res)
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[sweet::test]
	async fn works() -> Result<()> {
		let res = fetch("https://example.com").await?;
		expect(res.status()).to_be(200);
		Ok(())
	}
	#[sweet::test]
	async fn text() -> Result<()> {
		let res = fetch("https://example.com").await?;
		let text = res.x_text().await?;
		expect(text.as_str())
			.to_contain("This domain is for use in illustrative examples");
		expect(res.status()).to_be(200);
		Ok(())
	}
}
