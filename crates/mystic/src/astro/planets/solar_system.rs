use super::*;
use std::collections::HashMap;
use strum::EnumCount;

#[derive(Debug, Clone)]
pub struct SolarSystem {
	pub day: Y2000Day,
	pub bodies: HashMap<Body, OrbitalBody>,
}




impl SolarSystem {
	pub fn new(day: Y2000Day) -> Self {
		let bodies = HashMap::with_capacity(Body::COUNT);
		Self { day, bodies }
	}

	pub fn gmst(&self) -> f64 {
		wrap_deg(self.bodies.get(&Body::Sun).unwrap().el.l * DEG2HOURS + 12.)
	}

	pub fn geocentric_ecliptic(&self) -> HashMap<Body, RectangluarCoords> {
		geocentric_ecliptic(self.day)
	}

	pub fn heliocentric_ecliptic(&self) -> HashMap<Body, RectangluarCoords> {
		heliocentric_ecliptic(self.day)
	}
	pub fn equatorial(&self) -> HashMap<Body, EquatorialCoords> {
		self.geocentric_ecliptic()
			.iter()
			.map(|(key, value)| (*key, value.to_equatorial()))
			.collect()
		// geocentric_ecliptic(&self.bodies, self.day)
	}

	pub fn topocentric(
		&self,
		latitude: f64,
		longitude: f64,
	) -> HashMap<Body, TopocentricCoords> {
		self.equatorial()
			.iter()
			.map(|(key, value)| {
				(
					*key,
					TopocentricCoords::from_equatorial(
						value,
						latitude,
						longitude,
						self.gmst(),
						self.day.utc_hour(),
					),
				)
			})
			.collect()
	}
}
