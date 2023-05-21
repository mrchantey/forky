use anyhow::Result;
use forky_core::wasm::on_animation_frame;
use std::sync::{Arc, Mutex};

use super::*;


//https://towardsdatascience.com/how-to-create-plot-in-rust-fdc6c024461c
pub fn hello_plot() -> Result<()> {
	let acc = Accelerometer::new();
	let gyro = Gyroscope::new();
	// let test = TestChart::new();
	// let test2 = TestChart::new();

	let charts: Vec<Box<dyn SensorChart>> = vec![
		// Box::new(test),
		// Box::new(test2),
		Box::new(acc),
		Box::new(gyro),
	];

	let layout = PlotLayout::new(charts)?;
	let layout = Arc::new(Mutex::new(layout));
	let run = move || -> Result<()> {
		let mut layout = layout.lock().unwrap();
		layout.update()?;
		layout.draw()?;

		Ok(())
	};
	on_animation_frame(move || run().unwrap());


	Ok(())
}
