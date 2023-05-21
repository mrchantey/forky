use super::*;
use anyhow::Result;


pub trait SensorChart {
	fn update(&mut self) -> Result<()>;
	fn draw(&self, plot_canvas: &PlotCanvas) -> Result<()>;
}
