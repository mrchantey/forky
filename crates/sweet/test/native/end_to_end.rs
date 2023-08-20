use fantoccini::ClientBuilder;
use serde_json::Value;
use sweet::*;

sweet! {
	it nonSend "works" {
		let c = ClientBuilder::native().connect("http://localhost:9515")
		.await.map_err(|e| anyhow::anyhow!("failed to connect, is chromedriver running?\n{:?}",e))?;

		let result = c.execute("return ['foo','bar',3]", Vec::new()).await?;
		let vec = value_to_vec_str(&result);

		println!("result: {:?}", vec);

		// c.goto("https://en.wikipedia.org/wiki/Foobar").await?;

		c.close().await?;
	}
}


fn value_to_vec_str(value: &Value) -> Vec<&str> {
	if let Some(vec) = value.as_array() {
		vec.iter().filter_map(|v| v.as_str()).collect()
	} else {
		Vec::new()
	}
}
