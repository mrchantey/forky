use super::*;
use anyhow::Result;
use js_sys::Date;
use forky_core::math::*;



pub struct TestChart {
	pub chart: PlotChartXYZ,
	pub start: f64,
}


impl TestChart {
	pub fn new() -> Self {
		let chart = PlotChartXYZ::new("Test Chart");

		Self {
			chart,
			start: Date::now(),
		}
	}
}

impl SensorChart for TestChart {
	#[rustfmt::skip]
	fn update(&mut self) -> Result<()> { 
		let elapsed = (Date::now() - self.start) as f32;
		self.chart.x.push_with_time(elapsed, (elapsed * 0.001 * TAU).sin());
		self.chart.y.push_with_time(elapsed, (elapsed * 0.001 * TAU + TAU / 3.).sin());
		self.chart.z.push_with_time(elapsed, (elapsed * 0.001 * TAU + TAU / 3. * 2.).sin());
		Ok(()) 
	}

	fn draw(&self, root: &PlotCanvas) -> Result<()> {
		self.chart.draw(&root)?;
		Ok(())
	}
}
