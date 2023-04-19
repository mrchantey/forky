use super::{ecliptic_positions::ecliptic_position, *};
use std::collections::HashMap;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug, Clone)]
pub struct SolarSystem {
	// pub day: Y2000Day,
}

impl SolarSystem {
	// pub fn new(day: Y2000Day) -> Self { Self { day } }

	// pub fn gmst(&self) -> f64 {
	// 	wrap_deg(self.bodies.get(&Body::Sun).unwrap().el.l * DEG2HOURS + 12.)
	// }

	pub fn geocentric_ecliptic(day: Y2000Day) -> HashMap<Planet, GeoCoords> {
		let planets = Self::heliocentric_ecliptic(day);
		let earth_pos = planets.get(&Planet::Earth).unwrap().clone();
		planets
			.iter()
			.map(|(key, value)| (*key, value.to_geo_with_earth(earth_pos)))
			.collect()
	}

	pub fn heliocentric_ecliptic(
		day: Y2000Day,
	) -> HashMap<Planet, HelioCoords> {
		let mut planets = HashMap::with_capacity(Planet::COUNT);
		for body in Planet::iter() {
			planets.insert(body, ecliptic_position(day, body));
		}
		planets
	}
	// pub fn equatorial(&self) -> HashMap<Body, EquatorialCoords> {
	// 	self.geocentric_ecliptic()
	// 		.iter()
	// 		.map(|(key, value)| (*key, value.to_equatorial()))
	// 		.collect()
	// 	// geocentric_ecliptic(&self.bodies, self.day)
	// }

	// pub fn horizontal(
	// 	&self,
	// 	position: &GeographicCoords,
	// ) -> HashMap<Body, HorizontalCoords> {
	// 	self.equatorial()
	// 		.iter()
	// 		.map(|(key, value)| (*key, value.to_horizontal(position, self.day)))
	// 		.collect()
	// }
}
