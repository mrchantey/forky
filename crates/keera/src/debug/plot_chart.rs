use std::{default, ops::Range};

use super::*;
use anyhow::Result;
use plotters::{coord::types::RangedCoordf32, prelude::*};
use plotters_canvas::CanvasBackend;


// const Y_RANGE: Range<f32> = -1f32..1f32;

pub type Chart<'a> = ChartContext<
	'a,
	CanvasBackend,
	Cartesian2d<RangedCoordf32, RangedCoordf32>,
>;

pub struct PlotChart {
	// pub lines: Vec<&'a PlotLine>,
	pub title: &'static str,
}


const COLOR: RGBColor = RGBColor(255, 255, 255);




impl PlotChart {
	pub fn new(title: &'static str) -> Self {
		Self {
			// lines: Vec::new(),
			title,
		}
	}

	pub fn x_range(&self, lines: &[&PlotLine]) -> Range<f32> {
		let ranges = lines.iter().map(|line| line.x_range());
		let mut min = f32::MAX;
		let mut max = f32::MIN;
		for range in ranges {
			if range.start < min {
				min = range.start;
			}
			if range.end > max {
				max = range.end;
			}
		}
		min..max
	}
	pub fn y_range(&self, lines: &[&PlotLine]) -> Range<f32> {
		let ranges = lines.iter().map(|line| line.y_range());
		let mut min = f32::MAX;
		let mut max = f32::MIN;
		for range in ranges {
			if range.start < min {
				min = range.start;
			}
			if range.end > max {
				max = range.end;
			}
		}
		min..max
	}

	pub fn draw(&self, root: &PlotCanvas, lines: &[&PlotLine]) -> Result<()> {
		let title_style: TextStyle =
			("sans-serif", 50).into_font().color(&COLOR).into();
		let label_style: TextStyle =
			("sans-serif", 15).into_font().color(&COLOR).into();
		let axis_style = ShapeStyle {
			color: RGBAColor(200, 200, 200, 1.),
			filled: true,
			stroke_width: 1,
		};
		let bold_style = ShapeStyle {
			color: RGBAColor(200, 200, 200, 0.75),
			filled: true,
			stroke_width: 1,
		};
		let light_style = ShapeStyle {
			color: RGBAColor(200, 200, 200, 0.0),
			filled: true,
			stroke_width: 1,
		};

		let x_range = self.x_range(lines);
		let y_range = self.y_range(lines);
		let mut chart = ChartBuilder::on(&root)
			.caption(self.title, title_style)
			.margin(20)
			.x_label_area_size(30)
			.y_label_area_size(30)
			.build_cartesian_2d(x_range, y_range)?;

		chart
			.configure_mesh()
			.axis_style(axis_style)
			.label_style(label_style.clone())
			.x_label_style(label_style.clone())
			.y_label_style(label_style.clone())
			.bold_line_style(bold_style)
			.light_line_style(light_style)
			.draw()?;

		for line in lines.iter() {
			line.draw(&mut chart)?;
		}

		chart
			.configure_series_labels()
			.position(SeriesLabelPosition::UpperRight)
			.background_style(&WHITE.mix(0.1))
			.border_style(&WHITE)
.label_font(label_style)
			.draw()?;
		Ok(())
	}
}
