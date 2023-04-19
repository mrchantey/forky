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

	pub fn to_rectangular(&self) -> RectangluarCoords {
		RectangluarCoords {
			x: self.radius * cos_d(self.longitude) * cos_d(self.latitude),
			y: self.radius * sin_d(self.longitude) * cos_d(self.latitude),
			z: self.radius * sin_d(self.latitude),
		}
	}
	
	pub fn from_rectangular(rect: &RectangluarCoords) -> Self {
		let xy_sq = rect.x * rect.x + rect.y * rect.y;
		let mut longitude = atan2_d(rect.y, rect.x);
		if longitude < 0. {
			longitude += 360.0;
		}
		let xy_len = xy_sq.sqrt();
		let latitude = atan2_d(rect.z, xy_len);
		let radius = rect.length();
		SphericalCoords {
			longitude,
			latitude,
			radius,
		}
		// this.Polar = function (x, y, z) {
			// 	var rho = (x * x) + (y * y);
			// 	var radius = Math.sqrt(rho + (z * z));
			// 	var phi = Angle.AtanDeg2(y, x);
		// 	var rho = Math.sqrt(rho);
		// 	var theta = Angle.AtanDeg2(z, rho);
		// 	return new SphericalCoordinates(phi, theta, radius);
		// }
	}
}

impl RectangluarCoords {
	pub fn to_spherical(&self) -> SphericalCoords {
		SphericalCoords::from_rectangular(self)
	}
}
