use super::*;
use anyhow::Result;
use plotters::prelude::*;



pub struct PlotChartXYZ {
	pub chart: PlotChart,
	pub x: PlotLine,
	pub y: PlotLine,
	pub z: PlotLine,
}


impl PlotChartXYZ {
	pub fn new(title: &'static str) -> Self {
		let chart = PlotChart::new(title);
		let x = PlotLine::new("X", &RGBColor(255, 0, 255));
		let y = PlotLine::new("Y", &RGBColor(255, 255, 0));
		let z = PlotLine::new("Z", &RGBColor(0, 255, 255));
		Self { chart, x, y, z }
	}

	pub fn draw(&self, root: &PlotCanvas) -> Result<()> {
		self.chart.draw(&root, &[&self.x, &self.y, &self.z])?;
		// self.chart.draw(&root, &[&self.x, &self.y])?;
		Ok(())
	}
}
