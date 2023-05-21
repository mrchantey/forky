use super::*;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub struct Gyroscope {
	pub chart: PlotChartXYZ,
	pub device_gyroscope: Arc<Mutex<DeviceGyroscope>>,
}

impl Gyroscope {
	pub fn new() -> Self {
		let chart = PlotChartXYZ::new("Gyroscope");
		Self {
			chart,
			device_gyroscope: DeviceGyroscope::new(),
		}
	}
}

impl SensorChart for Gyroscope {
	fn update(&mut self) -> Result<()> {
		let gyro = self.device_gyroscope.lock().unwrap();
		self.chart.x.push(gyro.x as f32);
		self.chart.y.push(gyro.y as f32);
		self.chart.z.push(gyro.z as f32);
		Ok(())
	}

	fn draw(&self, root: &PlotCanvas) -> Result<()> { self.chart.draw(&root) }
}
