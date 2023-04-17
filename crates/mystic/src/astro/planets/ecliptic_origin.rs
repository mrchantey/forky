use super::*;
use std::collections::{HashMap, HashSet};


fn geo_bodies() -> HashSet<Body> { HashSet::from([Body::Sun, Body::Moon]) }

/// Calculate geocentric positions. For the sun and moon this is already the case, otherwise add the suns position.
pub fn geocentric_ecliptic(
	bodies: &HashMap<Body, OrbitalBody>,
) -> HashMap<Body, RectangluarCoords> {
	let geo_bodies = geo_bodies();

	bodies
		.iter()
		.map(|(key, value)| {
			if geo_bodies.contains(key) {
				(*key, value.ecliptic_rect)
			} else {
				(
					*key,
					value.ecliptic_rect
						+ bodies.get(&Body::Sun).unwrap().ecliptic_rect,
				)
			}
		})
		.collect()
	//convert moon radius to AU
	// bodies.moon.eclipticSphere.radius /= earthRadiiPerAU
	// bodies.moon.eclipticRect = eclipticalSphereToRectangular(bodies.moon.eclipticSphere)
}
pub fn heliocentric_ecliptic(
	bodies: &HashMap<Body, OrbitalBody>,
) -> HashMap<Body, RectangluarCoords> {
	let geo_bodies = geo_bodies();

	bodies
		.iter()
		.map(|(key, value)| {
			if geo_bodies.contains(key) {
				(
					*key,
					value.ecliptic_rect
						- bodies.get(&Body::Sun).unwrap().ecliptic_rect,
				)
			} else {
				(*key, value.ecliptic_rect)
			}
		})
		.collect()
}
