use super::*;
use crate::astro::*;


pub struct Chart;


impl Chart {
	pub fn new(day: Y2000Day) -> Self {
		let solar_system = planets::SolarSystem::new(day);

		Chart {}
	}
}
