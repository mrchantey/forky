#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeographicCoords {
	pub latitude: f64,
	pub longitude: f64,
	pub altitude: f64,
}

pub const GEOGRAPHIC_COORDS_SYDNEY: GeographicCoords = GeographicCoords {
	latitude: -33.8688,
	longitude: 151.2093,
	altitude: 0.,
};


impl GeographicCoords {
	pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
		Self {
			latitude,
			longitude,
			altitude,
		}
	}
	pub fn latlong(latitude: f64, longitude: f64) -> Self {
		Self::new(latitude, longitude, 0.)
	}
}
