use super::{ecliptic_positions::ecliptic_position, *};
use std::collections::HashMap;
use strum::{EnumCount, IntoEnumIterator};

pub fn geocentric_ecliptic(day: Y2000Day) -> HashMap<Body, RectangluarCoords> {
	let mut planets = heliocentric_ecliptic(day);
	let earth_pos = planets.get(&Body::Earth).unwrap().clone();
	for (_, mut planet) in planets.iter_mut() {
		planet.x -= earth_pos.x;
		planet.y -= earth_pos.y;
		planet.z -= earth_pos.z;
	}
	planets
}
pub fn heliocentric_ecliptic(
	day: Y2000Day,
) -> HashMap<Body, RectangluarCoords> {
	let mut planets = HashMap::with_capacity(Body::COUNT);
	for body in Body::iter() {
		planets.insert(Body::Sun, ecliptic_position(day, body));
	}
	planets
}
