use super::super::*;
use super::*;


pub fn ecliptic_position(day: Y2000Day, body: Body) -> HelioCoords {
	match body {
		Body::Sun => sun(),
		Body::Moon => moon(day),

		Body::Mercury => mercury(day),
		Body::Venus => venus(day),
		Body::Mars => mars(day),
		Body::Earth => earth(day),
		Body::Jupiter => jupiter(day),
		Body::Saturn => saturn(day),
		Body::Uranus => uranus(day),
		Body::Neptune => neptune(day),
		Body::Pluto => pluto(day),
	}
}
