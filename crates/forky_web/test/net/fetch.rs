use forky_web::*;
use sweet::*;

sweet! {
	test "fetch" {
		let res = fetch("/__ping__").await?;
		expect(res.status()).to_be(200)?;
	}
	test "text" {
		let res = fetch("/__ping__").await?;
		let text = res.x_text().await?;
		expect(text.as_str()).to_contain("__ping__")?;
	}
}
