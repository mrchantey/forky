use fantoccini::ClientBuilder;
use serde_json::Value;
use std::time::Duration;
use std::time::Instant;
use sweet::*;

sweet! {
	it nonSend "works" {
		let c = ClientBuilder::native().connect("http://localhost:9515")
		.await?;
	c.goto("http://localhost:7777").await?;

		// let result = c.execute("return ['foo','bar',3]", Vec::new()).await?;
		let start_time = Instant::now();
		loop{
			let result = c.execute("return window._sweet_log", Vec::new()).await?;
			let vec = value_to_vec_str(&result);
			if vec.last() == Some(&"_sweet_end") {
				break;
			}
			if start_time.elapsed() > Duration::from_secs(10) {
				panic!("timeout");
			}
			let result = vec.join("\n");
			println!("\nresult:\n{}", result);
			std::thread::sleep(Duration::from_secs(1));
		}
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
