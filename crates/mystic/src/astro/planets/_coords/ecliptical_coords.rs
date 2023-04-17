use super::super::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EclipticalCoords {
	pub longitude: f64,
	pub latitude: f64,
	pub radius: f64,
}

impl EclipticalCoords {
	pub fn to_rectangular(&self) -> RectangluarCoords {
		RectangluarCoords {
			x: self.radius * cos_d(self.longitude) * cos_d(self.latitude),
			y: self.radius * sin_d(self.longitude) * cos_d(self.latitude),
			z: self.radius * sin_d(self.latitude),
		}
	}
}
