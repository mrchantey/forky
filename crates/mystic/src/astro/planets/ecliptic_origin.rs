use super::{ecliptic_positions::ecliptic_position, *};
use std::collections::HashMap;
use strum::{EnumCount, IntoEnumIterator};

pub fn geocentric_ecliptic(day: Y2000Day) -> HashMap<Body, GeoCoords> {
	let planets = heliocentric_ecliptic(day);
	let earth_pos = planets.get(&Body::Earth).unwrap().clone();
	planets
		.iter()
		.map(|(key, value)| (*key, value.to_geo_with_earth(earth_pos)))
		.collect()
}
pub fn heliocentric_ecliptic(day: Y2000Day) -> HashMap<Body, HelioCoords> {
	let mut planets = HashMap::with_capacity(Body::COUNT);
	for body in Body::iter() {
		planets.insert(body, ecliptic_position(day, body));
	}
	planets
}
