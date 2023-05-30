use super::*;
use anyhow::Result;
use js_sys::Date;
use plotters::prelude::*;
use std::ops::Range;

const MAX_DISPLAYED_VALUES: usize = 512;
const MAX_VALUES: usize = MAX_DISPLAYED_VALUES * 2;
const X_MIN: f32 = 0.0;
const X_MAX: f32 = 10.0;
const Y_MIN: f32 = -0.001;
const Y_MAX: f32 = 0.001;


pub struct PlotLine {
	pub values: Vec<(f32, f32)>,
	pub title: &'static str,
	pub color: &'static RGBColor,
	pub started: f64,
}

impl PlotLine {
	pub fn new(title: &'static str, color: &'static RGBColor) -> Self {
		Self {
			values: Vec::new(),
			title,
			color,
			started: Date::now(),
		}
	}

	pub fn push_with_time(&mut self, x: f32, y: f32) {
		self.values.push((x, y));
		if self.values.len() > MAX_VALUES {
			self.values.remove(0);
		}
	}

	pub fn push(&mut self, y: f32) {
		let now = Date::now();
		self.push_with_time((now - self.started) as f32 * 0.001, y);
	}

	fn get_displayble<'a>(&'a self) -> &'a [(f32, f32)] {
		let min = std::cmp::min(MAX_DISPLAYED_VALUES, self.values.len());
		&self.values[self.values.len() - min..]
	}

	//take only the displayable values, for running graphs
	pub fn x_range(&self) -> Range<f32> {
		let displayable = self.get_displayble();

		let min = displayable
			.iter()
			.map(|(x, _y)| x)
			.min_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap_or(&X_MIN);
		let max = displayable
			.iter()
			.map(|(x, _y)| x)
			.max_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap_or(&X_MAX);

		let min = f32::max(*min, X_MIN); //we want the max here for running graph
		let max = f32::max(*max, X_MAX);
		min..max
	}

	/// take the total range of all values, so not so much jumping around
	pub fn y_range(&self) -> Range<f32> {
		let min = self
			.values
			.iter()
			.map(|(_x, y)| y)
			.min_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap_or(&Y_MIN);
		let max = self
			.values
			.iter()
			.map(|(_x, y)| y)
			.max_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap_or(&Y_MAX);
		let min = f32::min(*min, Y_MIN);
		let max = f32::max(*max, Y_MAX);

		min..max
		// *min..*max
	}

	pub fn draw(&self, chart: &mut Chart) -> Result<()> {
		let col = self.color.clone();
		let values = self.get_displayble().iter().map(|(x, y)| (*x, *y));
		chart
			.draw_series(LineSeries::new(values, col))?
			.label(self.title)
			.legend(move |(x, y)| {
				PathElement::new(vec![(x, y), (x + 20, y)], col)
			});
		Ok(())
	}
}
