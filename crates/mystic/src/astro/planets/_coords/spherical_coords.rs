use super::super::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SphericalCoords {
	pub longitude: f64,
	pub latitude: f64,
	pub radius: f64,
}


impl SphericalCoords {
	pub fn new(longitude: f64, latitude: f64, radius: f64) -> Self {
		SphericalCoords {
			longitude,
			latitude,
			radius,
		}
	}

	pub fn to_rectangular(&self) -> RectCoords {
		RectCoords {
			x: self.radius * cos_d(self.longitude) * cos_d(self.latitude),
			y: self.radius * sin_d(self.longitude) * cos_d(self.latitude),
			z: self.radius * sin_d(self.latitude),
		}
	}

	pub fn from_rectangular(rect: &RectCoords) -> Self {
		SphericalCoords {
			radius: rect.length(),
			longitude: wrap_deg(atan2_d(rect.y, rect.x)),
			latitude: atan2_d(rect.z, rect.length_xy()),
		}
	}
}

impl std::fmt::Display for SphericalCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"longitude: {:.prec$}, latitude: {:.prec$}, radius: {:.prec$})",
			self.longitude,
			self.latitude,
			self.radius,
			prec = DISPLAY_PRECISION
		)
	}
}

impl RectCoords {
	pub fn to_spherical(&self) -> SphericalCoords {
		SphericalCoords::from_rectangular(self)
	}
}
