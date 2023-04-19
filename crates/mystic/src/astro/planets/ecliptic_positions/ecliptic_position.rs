use super::super::*;
use super::*;


pub fn ecliptic_position(day: Y2000Day, body: Planet) -> HelioCoords {
	match body {
		Planet::Sun => sun(),
		Planet::Moon => moon(day),

		Planet::Mercury => mercury(day),
		Planet::Venus => venus(day),
		Planet::Mars => mars(day),
		Planet::Earth => earth(day),
		Planet::Jupiter => jupiter(day),
		Planet::Saturn => saturn(day),
		Planet::Uranus => uranus(day),
		Planet::Neptune => neptune(day),
		Planet::Pluto => pluto(day),
	}
}
