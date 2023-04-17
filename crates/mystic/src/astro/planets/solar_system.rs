use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SolarSystem {
	pub day: Y2000Day,
	pub bodies: HashMap<Body, OrbitalBody>,
}




impl SolarSystem {
	pub fn new(day: Y2000Day) -> Self {
		let mut bodies = get_orbital_constants()
			.iter()
			.map(|(key, value)| {
				let el = OrbitalElements::new(value, day);
				let body = OrbitalBody::new(el);
				(*key, body)
			})
			.collect();
		apply_pertubations(&mut bodies);

		Self { day, bodies }
	}

	pub fn gmst(&self) -> f64 {
		wrapDeg(self.bodies.get(&Body::Sun).unwrap().el.L * deg2hours + 12.)
	}

	pub fn geocentric_ecliptic(&self) -> HashMap<Body, RectangluarCoords> {
		geocentric_ecliptic(&self.bodies)
	}

	pub fn heliocentric_ecliptic(&self) -> HashMap<Body, RectangluarCoords> {
		heliocentric_ecliptic(&self.bodies)
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
