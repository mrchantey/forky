use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GeographicCoords {
	pub latitude: f64,
	pub longitude: f64,
	pub altitude: f64,
}

impl GeographicCoords {
	pub const GREENWICH:GeographicCoords = GeographicCoords {
		latitude: 51.477928,
		longitude: 0.,
		altitude: 1.,
	};
	pub const SYDNEY:GeographicCoords = GeographicCoords {
		latitude: -33.8688,
		longitude: 151.2093,
		altitude: 0.,
	};

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
