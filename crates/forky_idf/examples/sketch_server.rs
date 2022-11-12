use anyhow::Result;
use forky_idf::*;


fn main() -> Result<()> {
	SketchServer::run()?;
	// sketch.run();
	// // Arc::clone(&leds).lock().unwrap().show();
	// println!("sketch ok!");
	Ok(())
}
