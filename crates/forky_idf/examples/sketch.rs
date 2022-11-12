use anyhow::Result;
use forky_idf::*;


fn main() -> Result<()> {
	let mut sketch = default_sketch()?;
	sketch.run();
	// Arc::clone(&leds).lock().unwrap().show();
	println!("sketch ok!");
	Ok(())
}
