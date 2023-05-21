use super::*;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub struct Accelerometer {
	pub chart: PlotChartXYZ,
	pub device_accelerometer: Arc<Mutex<DeviceAccelerometer>>,
}

impl Accelerometer {
	pub fn new() -> Self {
		let chart = PlotChartXYZ::new("Accelerometer");
		Self {
			chart,
			device_accelerometer: DeviceAccelerometer::new(),
		}
	}
}

impl SensorChart for Accelerometer {
	fn update(&mut self) -> Result<()> {
		let acc = self.device_accelerometer.lock().unwrap();
		self.chart.x.push(acc.x as f32);
		self.chart.y.push(acc.y as f32);
		self.chart.z.push(acc.z as f32);
		Ok(())
	}

	fn draw(&self, root: &PlotCanvas) -> Result<()> { self.chart.draw(&root) }
}
