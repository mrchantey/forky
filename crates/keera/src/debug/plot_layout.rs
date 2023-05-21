use super::*;
use anyhow::Result;
use forky_core::wasm::*;
use plotters::{coord::Shift, prelude::*};
use plotters_canvas::CanvasBackend;


pub type PlotCanvas = DrawingArea<CanvasBackend, Shift>;


pub struct PlotLayout {
	pub charts: Vec<Box<dyn SensorChart>>,
	pub sections: Vec<PlotCanvas>,
	pub plot_canvas: PlotCanvas,
	pub background_color: RGBColor,
}



impl PlotLayout {
	pub fn new(charts: Vec<Box<dyn SensorChart>>) -> Result<Self> {
		let canvas = create_canvas().unwrap();
		let canvas_id = "plotter-canvas";
		canvas.set_attribute("width", "800").unwrap();
		canvas.set_attribute("height", "1000").unwrap();

		canvas.set_id(canvas_id);

		let background_color = RGBColor(32, 32, 32);

		let backend =
			CanvasBackend::new(canvas_id).expect("cannot find canvas");
		let plot_canvas = backend.into_drawing_area();

		Ok(Self {
			sections: plot_canvas.split_evenly((charts.len(), 1)),
			charts,
			background_color,
			plot_canvas,
		})
	}

	fn clear(&self) -> Result<()> {
		self.plot_canvas.fill(&self.background_color)?;
		Ok(())
	}

	pub fn update(&mut self) -> Result<()> {
		for chart in &mut self.charts {
			chart.update()?;
		}
		Ok(())
	}

	pub fn draw(&self) -> Result<()> {
		self.clear()?;
		// let (upper, lower) = root.split_vertically((50).percent());

		for (i, chart) in self.charts.iter().enumerate() {
			chart.draw(&self.sections[i])?;
		}
		self.plot_canvas.present()?;

		Ok(())
	}
}
